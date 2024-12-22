// log for android
#[macro_use] extern crate log;
slint::include_modules!();
//mod bluetooth;



#[cfg(target_os = "android")]
#[no_mangle]
pub fn init_android(app: slint::android::AndroidApp){
    info!("init_android");
    // ndk context
    use jni::JNIEnv;
    use jni::objects::JObject;
    use jni::sys::jobject;
    use jni::objects::JValue;

    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let mut env = vm.attach_current_thread().unwrap();

    // method 2
    // https://github.com/slint-ui/slint/issues/5839
    // https://github.com/slint-ui/slint/discussions/5692
    info!("set permissions");
    
    let p_bluetooth_scan = env.new_string("Manifest.permission.BLUETOOTH_SCAN").unwrap();
    let p_coarse_location = env.new_string("android.permission.ACCESS_COARSE_LOCATION").unwrap();
    let p_fine_location = env.new_string("android.permission.ACCESS_FINE_LOCATION").unwrap();
    let p_bluetooth_connect = env.new_string("android.permission.BLUETOOTH_CONNECT").unwrap();

    let string_class = env.find_class("java/lang/String").unwrap();
    let default_string = env.new_string("").unwrap();
    let perms = env.new_object_array(4, string_class, default_string).unwrap();
    env.set_object_array_element(&perms, 0, p_bluetooth_scan).unwrap();
    env.set_object_array_element(&perms, 1, p_bluetooth_connect).unwrap();
    env.set_object_array_element(&perms, 2, p_coarse_location).unwrap();
    env.set_object_array_element(&perms, 3, p_fine_location).unwrap();
    

    let brr = (&perms).into();
    let activity = unsafe { JObject::from_raw(app.activity_as_ptr() as jobject) };

    //info!("{:?}, {:?}, {:?}", perms, brr, activity);

    // android java permissions code
    // int permissionsCode = 42;
    // String[] permissions = {
    //         Manifest.permission.BLUETOOTH_SCAN,
    //         Manifest.permission.ACCESS_COARSE_LOCATION,
    //         Manifest.permission.ACCESS_FINE_LOCATION,
    //         Manifest.permission.BLUETOOTH_CONNECT
    // };
    // ActivityCompat.requestPermissions(a, permissions, permissionsCode);
    // let p = env.call_method(
    //     activity,
    //     "requestPermissions",
    //     "([Ljava/lang/String;II)V",
    //     &[(&perms).into(), JValue::from(0), JValue::from(0)]
    // ).unwrap();

    let p = env.call_method(
            activity,
            "requestPermissions",
            "([Ljava/lang/String;I)V",
            &[brr, jni::objects::JValueGen::Int(1)],
        ).unwrap();
    info!("{:?}", p);
    info!("init_android ok");


    // get jvm pointer

    use jni::sys::{JNI_GetCreatedJavaVMs, JNIInvokeInterface_};
    //use jni::JNIEnv;
    use std::ptr::null_mut;
    use jni::sys::{JavaVM, jsize};
    use jni::sys::jint;
    use jni::sys::JNI_OK;

    // let mut count: jint = 0;
    // let check = unsafe { JNI_GetCreatedJavaVMs(null_mut(), 0, &mut count) };
    // assert!(check == JNI_OK);
    // let mut vms = vec![null_mut(); count as usize];
    // let check = unsafe { JNI_GetCreatedJavaVMs(vms.as_mut_ptr(), vms.len() as i32, &mut count) };
    // assert!(check == JNI_OK);
    // assert!(vms.len() == count as usize);


    // let mut jvm_ptr = Vec::with_capacity(1);
    // let mut jvm: *mut JavaVM = null_mut();
    // let mut jvm_count: jsize = 0;
    // unsafe { JNI_GetCreatedJavaVMs(&mut jvm_ptr.as_mut_ptr(), 1, &mut jvm_count); };

    // Call JNI_GetCreatedJavaVMs to get the JVM pointer
    // let result = unsafe {
    //     JNI_GetCreatedJavaVMs(&mut jvm, 1, &mut jvm_count)
    // };

    // Now you can initialize btleplug with the JNIEnv
    //btleplug::platform::init(&env);
}


pub fn init() {
    // init blec bluetooth
    use blec::ble;
    let result = ble::init();
    match result {
        Ok(_) => {
            info!("Bluetooth initialized");
        }
        Err(e) => {
            info!("BLE Error: {}", e);
        }
    }
}


#[tokio::main] // async
pub async fn main() -> Result<(), slint::PlatformError> {
    // slint main window
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    // /*
    // ui.on_connect(move || {
    //     let my_ui = ui_weak.unwrap();
    //     // ....
    //     let _ = slint::spawn_local(async move {
    //         // ...
    //         let foobar = tokio::task::spawn(async move {
    //              // do the thing that needs to run on a tokio executor
    //              let foobar = ble::connect(
    //                 BleAddress::from_str_delim("00:11:22:33:44:55").unwrap(),
    //                 Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap(),
    //                 Vec<Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap()>,
    //                 {},
    //              ).await;
    //              foobar
    //         }).await;
    //         // now use foobar to set some property
    //         // ...
    //         my_ui.set_speed(15);
    //         });
    // });
    //  */


    // // async tasks
    // ui.on_connect(move || {
    //     let my_ui = ui_weak.unwrap();

    //     let _ = slint::spawn_local(async move {
    //         let foobar = tokio::task::spawn(async move {
    //              // do the thing that needs to run on a tokio executor
    //              let foobar = bluetooth::scan_stream().await;
    //              //let foobar = 48;
    //              foobar
    //         }).await;

    //         // now use foobar to set some property
    //         //my_ui.set_speed(foobar.unwrap());
    //     });
    // });


    // // this works for calling
    // ui.connect({
    //     move || {
    //         // Spawn thread to be able to call async functions.
    //         tokio::spawn(async move {

    //                 // Call async function
    //                 println!("here");
    //                 let item = bluetooth::scan_sleep().await;
    //                 /* 
    //                 // Update UI model state
    //                 update_model(
    //                     handle_weak.clone(),
    //                     item,
    //                 );*/
    //         });
    //     }
    // });


    // non async tasks
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_speed(ui.get_speed() + 1);
        }
    });

    ui.on_request_decrease_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_speed(ui.get_speed() - 1);
        }
    });

    // run slint ui
    ui.run()
}


#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    std::env::set_var("RUST_BACKTRACE", "1");

    // configure logcat
    extern crate android_logger;
    use log::LevelFilter;
    use android_logger::Config;

    android_logger::init_once(
        Config::default().with_max_level(LevelFilter::Info), // change log level here
    );

    info!("==== BIKE AID START ====");

    // init other
    slint::android::init(app.clone()).unwrap();
    init_android(app);
    init();

    // // main
    let _ = main(); // can check result if we get crashes
}


// #[cfg(target_os = "android")]
// #[no_mangle]
// fn activity(&self) -> JObject {
//     unsafe { JObject::from_raw(self.activity_as_ptr() as jni::sys::jobject) }
// }