use std::env;

fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
    let target = env::var("TARGET").unwrap();
    println!("cargo:warning=Target: {}", target);

}
