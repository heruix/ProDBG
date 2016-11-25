pub Application {
    funcs: *const GUApplicationFuncs,
    obj: *const GUApplication,
}

impl Application {
    pub fn run(&mut self) -> i32 {
        unsafe {
            ((*self.funcs).run)(self.obj)
        }
    }

}

pub Window {
    funcs: *const GUWindowFuncs,
    obj: *const GUWindow,
}

impl Window {
    pub fn set_title_foo(&mut self) {
        unsafe {
            ((*self.funcs).set_title_foo)(self.obj)
        }
    }

}

pub PushButton {
    funcs: *const GUPushButtonFuncs,
    obj: *const GUPushButton,
}

impl PushButton {
    pub fn set_default(&mut self, state: i32) {
        unsafe {
            ((*self.funcs).set_default)(self.obj, state)
        }
    }

}

pub MainWindow {
    funcs: *const GUMainWindowFuncs,
    obj: *const GUMainWindow,
}

impl MainWindow {
    pub fn add_dock_widget(&mut self, area: u32, widget: &DockWidget) {
        unsafe {
            ((*self.funcs).add_dock_widget)(self.obj, area, widget.get_obj())
        }
    }

}

pub DockWidget {
    funcs: *const GUDockWidgetFuncs,
    obj: *const GUDockWidget,
}

impl DockWidget {
    pub fn is_floating(&mut self) -> bool {
        unsafe {
            ((*self.funcs).is_floating)(self.obj)
        }
    }

    pub fn set_floating(&mut self, floating: bool) {
        unsafe {
            ((*self.funcs).set_floating)(self.obj, floating)
        }
    }

    pub fn set_widget(&mut self, widget: &Widget) {
        unsafe {
            ((*self.funcs).set_widget)(self.obj, widget.get_obj())
        }
    }

}

