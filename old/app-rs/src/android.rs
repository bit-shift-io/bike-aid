use jni::objects::GlobalRef;
use jni::{AttachGuard, JNIEnv, JavaVM};
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;
use jni::sys::{JNI_GetCreatedJavaVMs, JNIInvokeInterface_};
use std::ptr::null_mut;
use jni::sys::jsize;
use jni::sys::jint;
use jni::sys::JNI_OK;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::objects::JValue;
use jni::signature::{JavaType, Primitive};
use jni::objects::JString;
use jni_utils;

// log for android

 
pub static JAVAVM: OnceCell<JavaVM> = OnceCell::new();


// links
// https://github.com/alvr-org/ALVR/blob/17a79eadc926a2b9c701af6feefda12935f18e3c/alvr/system_info/src/android.rs#L48
// https://github.com/slint-ui/slint/issues/5839
// https://github.com/slint-ui/slint/discussions/5692


pub fn init(app: slint::android::AndroidApp) {
    std::env::set_var("RUST_BACKTRACE", "1");

    // configure logcat
    extern crate android_logger;
    use log::LevelFilter;
    use android_logger::Config;

    android_logger::init_once(
        Config::default().with_max_level(LevelFilter::Info), // change log level here
    );

    info!("==== BIKE AID START ANDROID ====");

    // init other 
    slint::android::init(app.clone()).unwrap();
    
    // init blec bluetooth
    //use blec::ble;
    // let result = ble::init();
    // match result {
    //     Ok(_) => {
    //         info!("Bluetooth initialized");
    //     }
    //     Err(e) => {
    //         info!("BLE Error: {}", e);
    //     }
    // }

    // device info
    let manufacturer = manufacturer_name();
    let model = model_name();
    let device = device_name();
    info!("{manufacturer} - {model} ({device})");

    // api level
    let api_level = get_api_level();
    info!("api: {}", api_level);

    // battery status
    let battery = get_battery_status();
    info!("battery: {:?}", battery);

    get_permission(&[
        "android.permission.BLUETOOTH_SCAN",
        "android.permission.BLUETOOTH_CONNECT",
        "android.permission.ACCESS_COARSE_LOCATION",
        "android.permission.ACCESS_FINE_LOCATION",
    ]);


    // // class loader
    // let mut env = get_env();
    // let activity = get_activity();
    // let activity_class = env.get_object_class(activity).unwrap();
    // let get_class_loader_method = resolve_method_id(env, activity_class, "getClassLoader", "()Ljava/lang/ClassLoader;")?;
    
    // trace!("Calling activity.getClassLoader()");


    
    
    // let class_loader = env.call_method(
    //     activity,
    //     "getClassLoader",
    //     "()Ljava/lang/ClassLoader;",
    //     &[],
    // ).unwrap()
    // .l().unwrap();


    // Now you can initialize btleplug with the JNIEnv
    //btleplug::platform::init(&env);
    //btleplug::platform::init(&env).unwrap();
}


pub fn get_vm() -> JavaVM {
    unsafe { JavaVM::from_raw(ndk_context::android_context().vm().cast()).unwrap() }
}


pub fn get_env() -> JNIEnv<'static> {
    let vm = get_vm();
    let env = vm.attach_current_thread().unwrap();
    let env_ptr = env.get_native_interface();
    unsafe { JNIEnv::from_raw(env_ptr as *mut _) }.unwrap()
}


pub fn get_context() -> jobject {
    ndk_context::android_context().context().cast()
}

pub fn get_activity() -> JObject<'static> {
    unsafe { JObject::from(get_context()) } // unsafe { JObject::from_raw(context()) }
}


fn get_api_level() -> i32 {
    let vm = get_vm();
    let mut env = vm.attach_current_thread().unwrap();

    env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")
        .unwrap()
        .i()
        .unwrap()
}

fn has_permissions(permissions: &[&str]) -> bool {
    let vm = get_vm();
    let mut env = vm.attach_current_thread().unwrap();

    for &permission in permissions {
        let perm_jstring = env.new_string(permission).unwrap();
        let permission_status = env
            .call_method(
                get_activity(),
                "checkSelfPermission",
                "(Ljava/lang/String;)I",
                &[JValue::from(perm_jstring)], // &[(&perm_jstring).into()],
            )
            .unwrap()
            .i()
            .unwrap();

        if permission_status != 0 {
            return false;
        }
    }

    true
}


pub fn get_permission(permissions: &[&str]) {
    let vm = get_vm();
    let mut env = vm.attach_current_thread().unwrap();

    let string_class = env.find_class("java/lang/String").unwrap();
    let default_string = env.new_string("").unwrap();
    let mut permissions_array = env.new_object_array(permissions.len() as i32, string_class, default_string).unwrap();

    for (i, &permission) in permissions.iter().enumerate() {
        let java_permission = env.new_string(permission).unwrap();
        env.set_object_array_element(permissions_array, i as i32, java_permission).unwrap(); // &mut permissions_array
    }

    if !has_permissions(permissions) {
        env.call_method(
            get_activity(),
            "requestPermissions",
            "([Ljava/lang/String;I)V",
            &[JValue::from(permissions_array), JValue::from(0)], // &[(&permissions_array).into(), 0.into()],
        ).unwrap();
    }


    info!("permissions: {:?}", has_permissions(permissions));
    // todo: handle case where permission is rejected
}



pub fn build_string(ty: &str) -> String {
    let vm = get_vm();
    let mut env = vm.attach_current_thread().unwrap();

    let jname = env
        .get_static_field("android/os/Build", ty, "Ljava/lang/String;")
        .unwrap()
        .l()
        .unwrap();
    //let name_raw = env.get_string((&jname).into()).unwrap();
    let jname: JString = jname.into();
    let name_raw = env.get_string(jname).unwrap();

    name_raw.to_string_lossy().as_ref().to_owned()
}


pub fn device_name() -> String {
    build_string("DEVICE")
}


pub fn model_name() -> String {
    build_string("MODEL")
}


pub fn manufacturer_name() -> String {
    build_string("MANUFACTURER")
}


pub fn get_battery_status() -> (f32, bool) {
    let vm = get_vm();
    let mut env = vm.attach_current_thread().unwrap();

    let intent_action_jstring = env
        .new_string("android.intent.action.BATTERY_CHANGED")
        .unwrap();
    let intent_filter = env
        .new_object(
            "android/content/IntentFilter",
            "(Ljava/lang/String;)V",
            &[JValue::from(intent_action_jstring)], // &[(&intent_action_jstring).into()],
        )
        .unwrap();
    let battery_intent = env
        .call_method(
            unsafe { JObject::from(get_context()) }, // unsafe { JObject::from_raw(context()) },
            "registerReceiver",
            "(Landroid/content/BroadcastReceiver;Landroid/content/IntentFilter;)Landroid/content/Intent;",
            &[JObject::null().into(), intent_filter.into()], // &[(&JObject::null()).into(), (&intent_filter).into()],
        )
        .unwrap()
        .l()
        .unwrap();

    let level_jstring = env.new_string("level").unwrap();
    let level = env
        .call_method(
            battery_intent, //&battery_intent,
            "getIntExtra",
            "(Ljava/lang/String;I)I",
            &[JValue::from(level_jstring), JValue::from(-1)] // &[(&level_jstring).into(), (-1).into()],
        )
        .unwrap()
        .i()
        .unwrap();
    let scale_jstring = env.new_string("scale").unwrap();
    let scale = env
        .call_method(
            battery_intent, //&battery_intent,
            "getIntExtra",
            "(Ljava/lang/String;I)I",
            &[JValue::from(scale_jstring), JValue::from(-1)] //&[(&scale_jstring).into(), (-1).into()],
        )
        .unwrap()
        .i()
        .unwrap();

    let plugged_jstring = env.new_string("plugged").unwrap();
    let plugged = env
        .call_method(
            battery_intent, // &battery_intent,
            "getIntExtra",
            "(Ljava/lang/String;I)I",
            &[JValue::from(plugged_jstring), JValue::from(-1)] //&[(&plugged_jstring).into(), (-1).into()],
        )
        .unwrap()
        .i()
        .unwrap();

    (level as f32 / scale as f32, plugged > 0)
}


// android jvm will automatically load this function when the library is loaded
#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, _res: *const std::os::raw::c_void) -> jni::sys::jint {
    let env = vm.get_env().unwrap();
    jni_utils::init(&env).unwrap();
    btleplug::platform::init(&env).unwrap();
    let _ = JAVAVM.set(vm);
    jni::JNIVersion::V6.into()
}
