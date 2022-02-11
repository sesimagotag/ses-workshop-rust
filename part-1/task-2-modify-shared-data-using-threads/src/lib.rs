use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(worker_count);

        for n in 0..worker_count {
            workers.push(Worker::new(n, Arc::clone(&rx)))
        }

        ThreadPool {
            workers,
            tx: Some(tx),
        }
    }

    pub fn execute<F>(&self, callback: F)
        where
            F: FnOnce() + Send + 'static,
    {
        match &self.tx {
            None => println!("No active workers anymore"),
            Some(tx) => tx.send(Box::new(callback))
                .expect("Thread shut down too early")
        }
    }

    pub fn join(&mut self) {
        self.tx = None;
        println!("Join active workers");
        for worker in self.workers.iter_mut() {
            worker.join();
        }
        println!("All workers finished");
    }
}

struct Worker {
    _id: usize,
    _handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let result = rx.lock().unwrap().recv();
            match result {
                Ok(rx) => {
                    println!("Worker {} got a job; executing.", id);
                    rx()
                }
                Err(_) => {
                    println!("Worker {} signing off", id);
                    break;
                }
            }
        });
        Worker {
            _id: id,
            _handle: Some(handle),
        }
    }

    fn join(&mut self) {
        self._handle.take().map(JoinHandle::join);
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, SystemTime};

    use super::*;

    #[test]
    fn test_threadpool() {
        let count = Arc::new(AtomicUsize::new(0));
        let worker_count = 4;
        let job_count = worker_count * 20;
        let intensive_work_time_millis = 100;
        let mut pool = ThreadPool::new(worker_count);
        let now = SystemTime::now();
        let elapsed = move || {
            now.elapsed().unwrap().as_millis()
        };

        for n in 0..job_count {
            let count = count.clone();
            pool.execute(move || {
                println!("{} - #{} working", elapsed(), n);
                // intensive work load
                thread::sleep(Duration::from_millis(intensive_work_time_millis));
                // increment shared counter
                println!("{} - #{} count++", elapsed(), n);
                count.fetch_add(1, Ordering::SeqCst);
            });
        }
        pool.join();
        let check_running_time = || {
            let mut now = elapsed();
            now -= now % intensive_work_time_millis as u128;
            now == (job_count * intensive_work_time_millis as usize / worker_count) as u128
        };
        assert!(check_running_time());
        pool.execute(move || {
            eprintln!("Should not be seen!");
        });

        let count = count.load(Ordering::SeqCst);

        assert_eq!(count, job_count);
    }
}
