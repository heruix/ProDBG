use std::os::raw::{c_void, c_int, c_char};

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUDockWidgetFuncs {
    pub create: unsafe extern "C" fn(parent: *mut GUWidget) -> *mut GUDockWidget,
    pub create_title: unsafe extern "C" fn(name: *const c_char, parent: *mut GUWidget) -> *mut GUDockWidget,
    pub is_floating: unsafe extern "C" fn(dock_widget: *mut GUDockWidget) -> u8,
    pub set_floating: unsafe extern "C" fn(dock_widget: *mut GUDockWidget, floating: u8),
    pub set_widget: unsafe extern "C" fn(dock_widget: *mut GUDockWidget, widget: *mut GUWidget),
    pub widget: unsafe extern "C" fn(dock_widget: *mut GUDockWidget) -> *mut GUWidget,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUObjectFuncs {
    pub connect: unsafe extern "C" fn(sender: *mut c_void, id: *const c_char, reciver: *mut c_void, func: *mut c_void) -> c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUWidgetFuncs {
    pub set_size: unsafe extern "C" fn(widget: *mut GUWidget, width: c_int, height: c_int),
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUMainWindowFuncs {
    pub add_dock_widget: unsafe extern "C" fn(win: *mut GUMainWindow, area: u32, widget: *mut GUDockWidget),
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUPushButtonFuncs {
    pub set_default: unsafe extern "C" fn(button: *mut GUPushButton, state: c_int),
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUApplicationFuncs {
    pub run: unsafe extern "C" fn(p: *mut c_void) -> c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUObject {
    pub p: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUWidget {
    pub object: *mut GUObject,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUDockWidget {
    pub base: *mut GUWidget,
    pub priv_: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUWindow {
    pub base: *mut GUWidget,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUPushButton {
    pub base: *mut GUWidget,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUMainWindow {
    pub base: *mut GUWidget,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct GUApplication {
    pub p: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Wrui {
    pub api_version: u64,
    pub application_create: extern "C" fn() -> *mut GUApplication,
    pub window_create: unsafe extern "C" fn(parent: *mut GUWidget) -> *mut GUWindow,
    pub push_button_create: unsafe extern "C" fn(label: *const c_char, parent: *const GUWidget) -> *mut GUPushButton,
    pub object_funcs: *mut GUObjectFuncs,
    pub widget_funcs: *mut GUWidgetFuncs,
    pub main_window_funcs: *mut GUMainWindowFuncs,
    pub push_button_funcs: *mut GUPushButtonFuncs,
    pub application_funcs: *mut GUApplicationFuncs,
}

extern "C" {
    pub fn wrui_get() -> *mut Wrui;
}
