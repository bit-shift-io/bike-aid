slint::include_modules!();
use blec::{ble, BleAddress};
use uuid::Uuid;
mod bluetooth;

// reference here:
// https://github.com/MnlPhlp/blec

pub fn init() {
    // init blec bluetooth
    let result = ble::init();
    match result {
        Ok(_) => {
            println!("Bluetooth initialized");
        }
        Err(_) => {
            println!("Bluetooth not initialized");
        }
    }
}


#[tokio::main] // async
pub async fn main() -> Result<(), slint::PlatformError> {
    // slint main window
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    /*
    ui.on_connect(move || {
        let my_ui = ui_weak.unwrap();
        // ....
        let _ = slint::spawn_local(async move {
            // ...
            let foobar = tokio::task::spawn(async move {
                 // do the thing that needs to run on a tokio executor
                 let foobar = ble::connect(
                    BleAddress::from_str_delim("00:11:22:33:44:55").unwrap(),
                    Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
                    Vec<Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap()>,
                    {},
                 ).await;
                 foobar
            }).await;
            // now use foobar to set some property
            // ...
            my_ui.set_speed(15);
            });
    });
     */


     /*
    ui.on_connect(move || {
        let my_ui = ui_weak.unwrap();
        // ....
        let _ = slint::spawn_local(async move {
            // ...
            let foobar = tokio::task::spawn(async move {
                 // do the thing that needs to run on a tokio executor
                 let foobar = bluetooth::scan_stream().await;
                 foobar
            }).await;
            // now use foobar to set some property
            // ...
            my_ui.set_speed(15);
            });
    }); */

   
    /*
    // this works for calling
    ui.on_scan({
        move || {
            // Spawn thread to be able to call async functions.
            tokio::spawn(async move {

                    // Call async function
                    println!("here");
                    let item = bluetooth::scan_sleep().await;
                    /* 
                    // Update UI model state
                    update_model(
                        handle_weak.clone(),
                        item,
                    );*/
            });
        }
    });
     */

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
    init();
    main();
}