use std::ffi::CString

pub Application {
    funcs: *const GUApplicationFuncs,
    obj: *const GUApplication,
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

pub Window {
    funcs: *const GUWindowFuncs,
    obj: *const GUWindow,
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

pub PushButton {
    funcs: *const GUPushButtonFuncs,
    obj: *const GUPushButton,
}

impl PushButton {
    pub fn set_default(&self, state: i32) {
        unsafe {
            ((*self.funcs).set_default)(self.obj, state)
        }
    }

    #[inline]
    pub fn get_obj(&self) -> *const GUPushButton { self.obj }
}

pub MainWindow {
    funcs: *const GUMainWindowFuncs,
    obj: *const GUMainWindow,
}

impl MainWindow {
    pub fn add_dock_widget(&self, area: u32, widget: &DockWidget) {
        unsafe {
            ((*self.funcs).add_dock_widget)(self.obj, area, widget.get_obj())
        }
    }

    #[inline]
    pub fn get_obj(&self) -> *const GUMainWindow { self.obj }
}

pub DockWidget {
    funcs: *const GUDockWidgetFuncs,
    obj: *const GUDockWidget,
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

