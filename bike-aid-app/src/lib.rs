slint::include_modules!();
mod bluetooth;


#[tokio::main]
pub async fn main() -> Result<(), slint::PlatformError> {
    // slint main window
    let ui = AppWindow::new()?;

    // async
    // https://github.com/slint-ui/slint/issues/747
    ui.on_scan({
        move || {
            // Spawn thread to be able to call async functions.
            tokio::spawn(async move {

                    // Call async function
                    println!("here");
                    let _ = bluetooth::scan_sleep().await;

                    // Update UI model state
                    //update_model(
                    //    handle_weak.clone(),
                    //    item,
                    //);
            });
        }
    });

    // non async example
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}


#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    main();
}