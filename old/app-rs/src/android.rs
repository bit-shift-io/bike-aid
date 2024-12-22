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

// links
// https://github.com/alvr-org/ALVR/blob/17a79eadc926a2b9c701af6feefda12935f18e3c/alvr/system_info/src/android.rs#L48
// https://github.com/slint-ui/slint/issues/5839
// https://github.com/slint-ui/slint/discussions/5692


pub fn init() {
    // device info
    let manufacturer = manufacturer_name();
    let model = model_name();
    let device = device_name();
    info!("{manufacturer} - {model} ({device})");

    // battery status
    let battery = get_battery_status();
    info!("battery: {:?}", battery);

    //try_get_permission("android.permission.BLUETOOTH_SCAN");
    try_get_permission("android.permission.BLUETOOTH_CONNECT");
    //try_get_permission("android.permission.ACCESS_COARSE_LOCATION");
    //try_get_permission("android.permission.ACCESS_FINE_LOCATION");

    // Now you can initialize btleplug with the JNIEnv
    //btleplug::platform::init(&env);

    // test



}


pub fn vm() -> JavaVM {
    unsafe { JavaVM::from_raw(ndk_context::android_context().vm().cast()).unwrap() }
}


pub fn context() -> jobject {
    ndk_context::android_context().context().cast()
}

pub fn activity() -> JObject<'static> {
    unsafe { JObject::from_raw(context()) }
}


fn get_api_level() -> i32 {
    let vm = vm();
    let mut env = vm.attach_current_thread().unwrap();

    env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")
        .unwrap()
        .i()
        .unwrap()
}


pub fn try_get_permission(permission: &str) {
    let vm = vm();
    let mut env = vm.attach_current_thread().unwrap();

    let mic_perm_jstring = env.new_string(permission).unwrap();

     // determine whether we have been granted a particular permission
    let permission_status = env
        .call_method(
            activity(),
            "checkSelfPermission",
            "(Ljava/lang/String;)I",
            &[(&mic_perm_jstring).into()],
        )
        .unwrap()
        .i()
        .unwrap();


    if permission_status != 0 {
        let string_class = env.find_class("java/lang/String").unwrap();
        let perm_array = env
            .new_object_array(1, string_class, mic_perm_jstring)
            .unwrap();

        let result = env.call_method(
            activity(),
            "requestPermissions",
            "([Ljava/lang/String;I)V",
            &[(&perm_array).into(), 0.into()],
        )
        .unwrap();

        
        info!("permission result: {:?}", result);

        // todo: handle case where permission is rejected
    }
}


pub fn build_string(ty: &str) -> String {
    let vm = vm();
    let mut env = vm.attach_current_thread().unwrap();

    let jname = env
        .get_static_field("android/os/Build", ty, "Ljava/lang/String;")
        .unwrap()
        .l()
        .unwrap();
    let name_raw = env.get_string((&jname).into()).unwrap();

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
    let vm = vm();
    let mut env = vm.attach_current_thread().unwrap();

    let intent_action_jstring = env
        .new_string("android.intent.action.BATTERY_CHANGED")
        .unwrap();
    let intent_filter = env
        .new_object(
            "android/content/IntentFilter",
            "(Ljava/lang/String;)V",
            &[(&intent_action_jstring).into()],
        )
        .unwrap();
    let battery_intent = env
        .call_method(
            unsafe { JObject::from_raw(context()) },
            "registerReceiver",
            "(Landroid/content/BroadcastReceiver;Landroid/content/IntentFilter;)Landroid/content/Intent;",
            &[(&JObject::null()).into(), (&intent_filter).into()],
        )
        .unwrap()
        .l()
        .unwrap();

    let level_jstring = env.new_string("level").unwrap();
    let level = env
        .call_method(
            &battery_intent,
            "getIntExtra",
            "(Ljava/lang/String;I)I",
            &[(&level_jstring).into(), (-1).into()],
        )
        .unwrap()
        .i()
        .unwrap();
    let scale_jstring = env.new_string("scale").unwrap();
    let scale = env
        .call_method(
            &battery_intent,
            "getIntExtra",
            "(Ljava/lang/String;I)I",
            &[(&scale_jstring).into(), (-1).into()],
        )
        .unwrap()
        .i()
        .unwrap();

    let plugged_jstring = env.new_string("plugged").unwrap();
    let plugged = env
        .call_method(
            &battery_intent,
            "getIntExtra",
            "(Ljava/lang/String;I)I",
            &[(&plugged_jstring).into(), (-1).into()],
        )
        .unwrap()
        .i()
        .unwrap();

    (level as f32 / scale as f32, plugged > 0)
}
