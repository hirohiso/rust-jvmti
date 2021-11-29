use std::{ffi::CStr, fmt::Debug};

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
    options: *const std::os::raw::c_char,
    _reserved: *const std::ffi::c_void,
) -> sys::jint {
    unsafe{
        if !options.is_null(){
            println!("not null");
            let option = CStr::from_ptr(options).to_str();
            if let Ok(v) = option{
                println!("{}",v);
            }
        }else{
            println!("null");
        }

        let mut jvmenv = std::ptr::null_mut();
        ((*(*vm)).GetEnv.unwrap())(vm,&mut jvmenv,sys::JVMTI_VERSION as i32);
        let jvmenv = jvmenv as *mut sys::jvmtiEnv;

        println!("===========================");
        let js = std::ptr::null_mut();
        let set_event_notification_mode = (**jvmenv).SetEventNotificationMode.unwrap();

        set_event_notification_mode(jvmenv,sys::jvmtiEventMode_JVMTI_ENABLE,sys::jvmtiEvent_JVMTI_EVENT_CLASS_PREPARE, js);
        set_event_notification_mode(jvmenv,sys::jvmtiEventMode_JVMTI_ENABLE,sys::jvmtiEvent_JVMTI_EVENT_CLASS_LOAD, js);
        set_event_notification_mode(jvmenv,sys::jvmtiEventMode_JVMTI_ENABLE,sys::jvmtiEvent_JVMTI_EVENT_CLASS_FILE_LOAD_HOOK,js);

        let mut capability  = sys::jvmtiCapabilities{
            ..Default::default()
        };
        capability.set_can_get_source_file_name(1);
        capability.set_can_generate_all_class_hook_events(1);
        
        ((**jvmenv).AddCapabilities.unwrap())(jvmenv,&capability);

        let callbacks = sys::jvmtiEventCallbacks{
            //ClassPrepare : Some(class_prepare),
            //ClassLoad : Some(class_load),
            ClassFileLoadHook: Some(class_file_load_hook),
            ..Default::default()
        };// 構造体更新記法でデフォルなところはこれで楽できる
        ((**jvmenv).SetEventCallbacks.unwrap())(jvmenv,&callbacks,std::mem::size_of::<sys::jvmtiEventCallbacks>() as i32);

    }
    println!("Hello, JVMTI!");
    0
}

#[no_mangle]
pub extern "C" fn Agent_OnUnload(){
    println!("Good bye.");
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
#[no_mangle]
unsafe extern "C" fn class_file_load_hook(
    jvmti_env: *mut sys::jvmtiEnv,
    jni_env: *mut sys::JNIEnv,
    class_being_redefined: sys::jclass,
    loader: sys::jobject,
    name: *const ::std::os::raw::c_char,
    protection_domain: sys::jobject,
    class_data_len: sys::jint,
    class_data: *const ::std::os::raw::c_uchar,
    new_class_data_len: *mut sys::jint,
    new_class_data: *mut *mut ::std::os::raw::c_uchar,
){
    if name == std::ptr::null_mut(){
        return;
    }

    let name = CStr::from_ptr(name);
    println!("Class file Load[{}][size={}]",name.to_str().unwrap().to_string(),class_data_len);
}

#[no_mangle]
unsafe extern "C" fn class_prepare(jvmti_env: *mut sys::jvmtiEnv,_jni_env: *mut sys::JNIEnv,_thread: sys::jthread,klass: sys::jclass,){
    let mut pchar = std::ptr::null_mut();
    let _ret = ((**jvmti_env).GetSourceFileName.unwrap())(jvmti_env,klass,&mut pchar);
    let pchar = CStr::from_ptr(pchar);
    println!("Class Prepare[{}]",pchar.to_str().unwrap().to_string());
}