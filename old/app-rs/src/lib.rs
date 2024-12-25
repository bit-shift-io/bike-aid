slint::include_modules!();
mod bluetooth;
mod app;

#[cfg(target_os = "android")]
mod android;

// log without needing to import
#[macro_use] extern crate log;


#[tokio::main]
pub async fn main() -> Result<(), slint::PlatformError> {
    app::init().await
}


#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    android::init(app);
    let _ = main();
}