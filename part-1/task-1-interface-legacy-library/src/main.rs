use legacy_binding::{add, ask, divide, multiply, subtract};

fn main() {
    println!("before");
    let val = ask(128);
    println!("{}", &val);
    println!("and after");

    let a = 7.4;
    let b = 99;

    println!("a + b = {}", add(a, f64::from(b)));
    println!("a - b = {}", subtract(a, f64::from(b)));
    println!("a * b = {}", multiply(a, f64::from(b)));
    println!("a / b = {:.7}", divide(a, f64::from(b)));
}
