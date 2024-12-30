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
    // not sure if we need this....
    // it looks to opcy stuff from the NDK to /home/bronson/Projects/bike-aid/old/app-rs/android/app/src/main/jniLibs/arm64-v8a/

    // println!("cargo:rustc-link-lib=c++_shared");

    // if let Ok(output_path) = env::var("CARGO_NDK_OUTPUT_PATH") {
    //     let sysroot_libs_path =
    //         PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
    //     let lib_path = sysroot_libs_path.join("libc++_shared.so");
    //     std::fs::copy(
    //         lib_path,
    //         Path::new(&output_path)
    //             .join(&env::var("CARGO_NDK_ANDROID_TARGET").unwrap())
    //             .join("libc++_shared.so"),
    //     )
    //     .unwrap();
    // }
}
