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
    println!("Hello, JVMTI!");
    0
}

#[no_mangle]
pub extern "C" fn Agent_OnUnload(){
    println!("Good bye.");
}