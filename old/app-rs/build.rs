use std::env;
use std::path::{Path, PathBuf};

fn main() {
    slint_build::compile("ui/appwindow.slint").expect("Slint build failed");
    // old
    //slint_build::compile("ui/appwindow.slint").unwrap();
    //let target = env::var("TARGET").unwrap();
    //println!("cargo:warning=Target: {}", target);
    
    // https://docs.rs/crate/cargo-ndk/latest
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}


fn android() {

}
