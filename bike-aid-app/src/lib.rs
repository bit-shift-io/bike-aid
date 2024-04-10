slint::include_modules!();

pub fn main() {
    let window = AppWindow::new().unwrap();
    window.run().unwrap();
}


#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    main();
}