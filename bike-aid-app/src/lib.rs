slint::include_modules!();
mod bluetooth;


pub fn main() {
    let window = AppWindow::new().unwrap();
    window.on_scan(do_scan);
    window.run().unwrap();
}


#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    main();
}


fn do_scan() {
    println!("here");
    bluetooth::main();
}

