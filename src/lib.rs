use std::ffi::CStr;

use libc::c_char;

#[repr(C)]
pub struct SDKHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct SHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct MHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct MRHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    // shim wrappers
    pub fn new_mavsdk() -> SDKHandle;
    pub fn del_mavsdk(p: SDKHandle);

    // helper wrappers
    pub fn connect(p: SDKHandle, addr: *const c_char) -> SHandle;
    pub fn run_qgc_plan(
        system: SHandle,
        path: *const c_char,
        cb_context: *mut Box<dyn RunCallBackObject>,
        cb_position: extern "C" fn(*mut Box<dyn RunCallBackObject>, f64, f64, f32),
        cb_progress: extern "C" fn(*mut Box<dyn RunCallBackObject>, i32, i32),
        cb_complete: extern "C" fn(*mut Box<dyn RunCallBackObject>),
        cb_log: extern "C" fn(*mut Box<dyn RunCallBackObject>, *const c_char),
    ) -> i32;
}

// run mode callback wrappers
pub extern "C" fn run_callback_position(
    context: *mut Box<dyn RunCallBackObject>,
    lat: f64,
    lon: f64,
    alt: f32,
) {
    unsafe {
        let cb: Box<Box<dyn RunCallBackObject>> = Box::from_raw(context);
        cb.save_position(lat, lon, alt);
    }
}

pub extern "C" fn run_callback_progress(
    context: *mut Box<dyn RunCallBackObject>,
    current: i32,
    total: i32,
) {
    unsafe {
        let cb: Box<Box<dyn RunCallBackObject>> = Box::from_raw(context);
        cb.save_progress(current, total);
    }
}

pub extern "C" fn run_callback_complete(context: *mut Box<dyn RunCallBackObject>) {
    unsafe {
        let cb: Box<Box<dyn RunCallBackObject>> = Box::from_raw(context);
        cb.complete();
    }
}

pub extern "C" fn run_callback_log(context: *mut Box<dyn RunCallBackObject>, msg: *const c_char) {
    unsafe {
        let cb: Box<Box<dyn RunCallBackObject>> = Box::from_raw(context);
        let c_str = CStr::from_ptr(msg).to_str().unwrap();
        cb.log(c_str);
    }
}

// run mode callback trait
pub trait RunCallBackObject {
    fn save_position(&self, lat: f64, lon: f64, alt: f32);
    fn save_progress(&self, current: i32, total: i32);
    fn log(&self, msg: &str);
    fn complete(&self);
}
