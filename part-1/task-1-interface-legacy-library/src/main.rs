use std::env;
use std::path::PathBuf;

#[allow(unused_imports)]
use legacy_binding::{add, ask, divide, multiply, subtract};

fn main() {
    println!("Task 1:");
    println!("\tcomplete the {:?} to output the same as", file!());
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("\tthe {:?} app.", &out_path.join("legacy"));
    println!("\n\tverify your progress with");
    println!("\t\tcargo run");
}
