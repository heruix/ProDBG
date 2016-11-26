use std::ffi::CString;

use ffi_gen::*;

use traits_gen::*;

pub struct Application {
    pub widget_funcs: *const GUWidgetFuncs,
    pub funcs: *const GUApplicationFuncs,
    pub obj: *const GUApplication,
}

impl Application {
    pub fn run(&self) -> i32 {
        unsafe {
            ((*self.funcs).run)(self.obj)
        }
    }

    #[inline]
    pub fn get_obj(&self) -> *const GUApplication { self.obj }
}

pub struct Window {
    pub widget_funcs: *const GUWidgetFuncs,
    pub funcs: *const GUWindowFuncs,
    pub obj: *const GUWindow,
}

impl Window {
    pub fn set_title(&self, name: &str) {
        let str_in_0 = CString::new(name).unwrap();
        unsafe {
            ((*self.funcs).set_title)(self.obj, str_in_0.as_ptr())
        }
    }

    #[inline]
    pub fn get_obj(&self) -> *const GUWindow { self.obj }
}

impl Widget for Window {
   fn get_obj(&self) -> *const GUWidget {
       unsafe { (*self.obj).base }
   }
   fn get_funcs(&self) -> *const GUWidgetFuncs {
       self.widget_funcs
   }
}

pub struct PushButton {
    pub widget_funcs: *const GUWidgetFuncs,
    pub funcs: *const GUPushButtonFuncs,
    pub obj: *const GUPushButton,
}

impl PushButton {
    pub fn set_title(&self, text: &str) {
        let str_in_0 = CString::new(text).unwrap();
        unsafe {
            ((*self.funcs).set_title)(self.obj, str_in_0.as_ptr())
        }
    }

    #[inline]
    pub fn get_obj(&self) -> *const GUPushButton { self.obj }
}

impl Widget for PushButton {
   fn get_obj(&self) -> *const GUWidget {
       unsafe { (*self.obj).base }
   }
   fn get_funcs(&self) -> *const GUWidgetFuncs {
       self.widget_funcs
   }
}

pub struct MainWindow {
    pub widget_funcs: *const GUWidgetFuncs,
    pub funcs: *const GUMainWindowFuncs,
    pub obj: *const GUMainWindow,
}

impl MainWindow {
    pub fn set_central_widget(&self, widget: &Widget) {
        unsafe {
            ((*self.funcs).set_central_widget)(self.obj, widget.get_obj())
        }
    }

    pub fn add_dock_widget(&self, area: u32, widget: &DockWidget) {
        unsafe {
            ((*self.funcs).add_dock_widget)(self.obj, area, widget.get_obj())
        }
    }

    #[inline]
    pub fn get_obj(&self) -> *const GUMainWindow { self.obj }
}

impl Widget for MainWindow {
   fn get_obj(&self) -> *const GUWidget {
       unsafe { (*self.obj).base }
   }
   fn get_funcs(&self) -> *const GUWidgetFuncs {
       self.widget_funcs
   }
}

pub struct DockWidget {
    pub widget_funcs: *const GUWidgetFuncs,
    pub funcs: *const GUDockWidgetFuncs,
    pub obj: *const GUDockWidget,
}

impl DockWidget {
    pub fn is_floating(&self) -> bool {
        unsafe {
            ((*self.funcs).is_floating)(self.obj)
        }
    }

    pub fn set_floating(&self, floating: bool) {
        unsafe {
            ((*self.funcs).set_floating)(self.obj, floating)
        }
    }

    pub fn set_widget(&self, widget: &Widget) {
        unsafe {
            ((*self.funcs).set_widget)(self.obj, widget.get_obj())
        }
    }

    #[inline]
    pub fn get_obj(&self) -> *const GUDockWidget { self.obj }
}

impl Widget for DockWidget {
   fn get_obj(&self) -> *const GUWidget {
       unsafe { (*self.obj).base }
   }
   fn get_funcs(&self) -> *const GUWidgetFuncs {
       self.widget_funcs
   }
}

