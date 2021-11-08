use std::ffi::{CStr, c_void};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod sys {
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
        ((*(*vm)).GetEnv.unwrap())(vm,&mut jvmenv,sys::JVMTI_VERSION_1_0 as i32);

        let mut jvmenv = jvmenv as sys::jvmtiEnv;
        ((*jvmenv).SetEventCallbacks.unwrap())(&mut jvmenv,std::ptr::null_mut(),0);

    }
    println!("Hello, JVMTI!");
    0
}

#[no_mangle]
pub extern "C" fn Agent_OnUnload(){
    println!("Good bye.");
}