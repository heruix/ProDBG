#![allow(dead_code)]

mod ffi_gen;
pub mod traits_gen;
pub mod widgets_gen;
pub mod ui_gen;

use std::os::raw::c_void;
use std::mem::transmute;
pub use ui_gen::Ui;
pub use traits_gen::Widget;

pub fn connect<D>(object: *const ffi_gen::GUObject, signal: &[u8], data: &D, fun: extern fn(*mut c_void)) {
    unsafe {
        ((*(*object).object_funcs).connect)(object as *const c_void, signal.as_ptr() as *const i8, transmute(data), fun as *const c_void);
    }
}

#[macro_export]
macro_rules! connect_released {
    ($sender:expr, $data:expr, $call_type:ident, $callback:path) => {
        {
            extern "C" fn temp_call(target: *mut std::os::raw::c_void) {
                unsafe {
                    let app = target as *mut $call_type; 
                    $callback(&mut *app);
                }
            }

            unsafe {
                let object = (*(*$sender.get_obj()).base).object;
                wrui::connect(object, b"2released()\0", $data, temp_call);
            }
        }
    }
}

