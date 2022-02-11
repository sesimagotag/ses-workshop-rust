use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::time::Duration;

use actix_web::{App, Error, HttpResponse, HttpServer, Responder, web};
use actix_web::rt::time::{Instant, interval_at};
use actix_web::web::{Bytes, Data};
use futures::{Stream, StreamExt};
use rusb::{Context as LibUsbContext, Device, HotplugBuilder, Registration, UsbContext};
use serde::Serialize;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("Hosting API at http://localhost:8080");
    let data = HotPlugHandler::create();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/devices", web::get().to(new_client))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn index() -> impl Responder {
    let content = include_str!("index.html");

    HttpResponse::Ok()
        .header("content-type", "text/html")
        .body(content)
}

async fn new_client(handler: Data<Mutex<HotPlugHandler>>) -> impl Responder {
    let rx = handler.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(rx)
}

#[derive(Debug, Clone, Serialize)]
struct DeviceInfo {
    bus: u8,
    address: u8,
    vendor_id: u16,
    product_id: u16,
    device_name: String,
    manufacturer: String,
    serial_number: String,
}

impl ToString for DeviceInfo {
    fn to_string(&self) -> String {
        format!("{:03}_{:03}_{:04x}_{:04x}",
                self.bus,
                self.address,
                self.vendor_id,
                self.product_id)
    }
}

impl<T: UsbContext> From<Device<T>> for DeviceInfo {
    fn from(device: Device<T>) -> Self {
        let device_desc = device.device_descriptor().unwrap();
        let (device_name, manufacturer, serial_number) = match device.open() {
            Ok(device_handle) => {
                let timeout = Duration::from_secs(1);
                let languages = device_handle.read_languages(timeout).unwrap();
                let language = languages[0];
                (
                    device_handle
                        .read_product_string(language, &device_desc, timeout)
                        .unwrap_or_default(),
                    device_handle
                        .read_manufacturer_string(language, &device_desc, timeout)
                        .unwrap_or_default(),
                    device_handle
                        .read_serial_number_string(language, &device_desc, timeout)
                        .unwrap_or_default()
                )
            }
            Err(_) => (String::default(), String::default(), String::default())
        };

        Self {
            bus: device.bus_number(),
            address: device.address(),
            vendor_id: device_desc.vendor_id(),
            product_id: device_desc.product_id(),
            device_name,
            manufacturer,
            serial_number,
        }
    }
}

struct HotPlugCallback {
    me: Data<Mutex<HotPlugHandler>>,
}

impl<T: UsbContext> rusb::Hotplug<T> for HotPlugCallback {
    fn device_arrived(&mut self, device: Device<T>) {
        self.me.lock().unwrap().device_arrived(&DeviceInfo::from(device));
    }

    fn device_left(&mut self, device: Device<T>) {
        self.me.lock().unwrap().device_left(&DeviceInfo::from(device).to_string());
    }
}

struct HotPlugHandler {
    clients: Vec<Sender<Bytes>>,
    devices: HashMap<String, DeviceInfo>,
}

impl HotPlugHandler {
    fn create() -> Data<Mutex<Self>> {
        let me = Data::new(Mutex::new(HotPlugHandler::new()));
        HotPlugHandler::spawn_hotplug_loop(me.clone());
        me
    }

    fn new() -> Self {
        HotPlugHandler {
            clients: Vec::new(),
            devices: HashMap::new(),
        }
    }

    fn spawn_hotplug_loop(me: Data<Mutex<Self>>) {
        actix_web::rt::spawn(async move {
            if rusb::has_hotplug() {
                let context = match LibUsbContext::new() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("usb hotplug failed: {}", e);
                        return;
                    }
                };

                let _reg: Option<Registration<LibUsbContext>> = Some(
                    match HotplugBuilder::new()
                        .enumerate(true)
                        .register(&context, Box::new(HotPlugCallback { me: me.clone() }))
                    {
                        Ok(r) => r,
                        Err(e) => {
                            eprintln!("usb hotplug failed: {}", e);
                            return;
                        }
                    }
                );

                let mut task = interval_at(Instant::now(), Duration::from_secs_f32(0.5));
                while task.next().await.is_some() {
                    if context.handle_events(None).is_err() {
                        return;
                    }
                }
            } else {
                eprintln!("libusb hotplug api unsupported");
            }
        })
    }

    fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        tx
            .try_send(Bytes::from("data: connected\n\n"))
            .unwrap();

        for (device_id, _device_info) in self.devices.iter() {
            let msg = Bytes::from(["data: ", format!("device arrived {}", device_id).as_str(), "\n\n"].concat());
            tx.clone()
                .try_send(msg.clone())
                .unwrap();
        }

        self.clients.push(tx);
        Client(rx)
    }

    fn send(&mut self, msg: &str) {
        let msg = Bytes::from(["data: ", msg, "\n\n"].concat());
        let mut ok_clients = Vec::new();
        for client in self.clients.iter() {
            let result = client.clone().try_send(msg.clone());

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    fn device_arrived(&mut self, device_info: &DeviceInfo) {
        self.devices.insert(device_info.to_string(), device_info.clone());
        self.send(format!("device arrived {}", device_info.to_string()).as_str());
    }

    fn device_left(&mut self, device_id: &String) {
        self.devices.remove(device_id);
        self.send(format!("device left {}", device_id).as_str());
    }
}

struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
