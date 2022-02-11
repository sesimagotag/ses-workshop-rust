use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=vendor/legacy++/mighty.h");

    let legacy_path = cmake::build("vendor/legacy++");
    let legacy_file = legacy_path.join("legacy");
    let output = Command::new(&legacy_file)
        .stdout(Stdio::piped())
        .output()
        .expect("Unable to call legacy");

    println!("cargo:rustc-link-search=native={}", &legacy_path.display());
    println!("cargo:rustc-link-lib=static=mighty");

    println!("cargo:warning=output from legacy app: {:?}", &legacy_file);

    for line in output.stdout.split(|&x| x == b'\n') {
        if line.is_empty() {
            continue
        }

        println!(
            "cargo:warning=\t{}",
            String::from_utf8_lossy(line)
        );
    }

    let bindings = bindgen::Builder::default()
        .header("vendor/legacy++/mighty.h")
        .clang_arg("-xc++")
        .clang_arg("-Ivendor/legacy++")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bind_file = out_path.join("bindings.rs");

    bindings
        .write_to_file(&bind_file)
        .expect("Couldn't write bindings!");

    let dest_dir = PathBuf::from("src").join("ffi");
    let dest_file = dest_dir.join("bindings.rs");

    std::fs::create_dir_all(&dest_dir).unwrap();
    std::fs::copy(&bind_file, &dest_file).unwrap();
}
