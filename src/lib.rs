use std::{ffi::CStr};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod sys {
    //https://rust-lang.github.io/rust-bindgen/tutorial-4.html
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


#[no_mangle]
pub extern "C" fn Agent_OnLoad(
    vm: *mut sys::JavaVM,
    _options: *const std::os::raw::c_char,
    _reserved: *const std::ffi::c_void,
) -> sys::jint {
    unsafe{

        let mut jvmenv = std::ptr::null_mut();
        ((*(*vm)).GetEnv.unwrap())(vm,&mut jvmenv,sys::JVMTI_VERSION as i32);
        let jvmenv = jvmenv as *mut sys::jvmtiEnv;

        let js = std::ptr::null_mut();
        let set_event_notification_mode = (**jvmenv).SetEventNotificationMode.unwrap();

        set_event_notification_mode(jvmenv,sys::jvmtiEventMode_JVMTI_ENABLE,sys::jvmtiEvent_JVMTI_EVENT_CLASS_LOAD, js);

        let mut capability  = sys::jvmtiCapabilities{
            ..Default::default()
        };
        capability.set_can_get_source_file_name(1);
        capability.set_can_generate_all_class_hook_events(1);
        
        ((**jvmenv).AddCapabilities.unwrap())(jvmenv,&capability);

        let callbacks = sys::jvmtiEventCallbacks{
            ClassLoad : Some(class_load),
            ..Default::default()
        };// 構造体更新記法でデフォルなところはこれで楽できる
        ((**jvmenv).SetEventCallbacks.unwrap())(jvmenv,&callbacks,std::mem::size_of::<sys::jvmtiEventCallbacks>() as i32);

    }
    0
}

#[no_mangle]
pub extern "C" fn Agent_OnUnload(){
}
#[no_mangle]
unsafe extern "C" fn class_load(jvmti_env: *mut sys::jvmtiEnv,_jni_env: *mut sys::JNIEnv,_thread: sys::jthread,klass: sys::jclass,){
    let mut pchar = std::ptr::null_mut();
    let _ret = ((**jvmti_env).GetSourceFileName.unwrap())(jvmti_env,klass,&mut pchar);
    let pchar = CStr::from_ptr(pchar);

    let mut pchar1 = std::ptr::null_mut();
    let mut pchar2 = std::ptr::null_mut();
    let _ret = ((**jvmti_env).GetClassSignature.unwrap())(jvmti_env,klass,&mut pchar1,&mut pchar2);

    let pchar1 = CStr::from_ptr(pchar1);
    println!("Class Load[{}][{}]",pchar.to_str().unwrap().to_string(),pchar1.to_str().unwrap().to_string());
}
