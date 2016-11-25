
use ffi_gen::*;
use widgets_gen::*;
pub struct Ui {
    wrui: *const Wrui
}

impl Ui {
    pub fn new(wrui: *const Wrui) -> Ui { Ui { wrui: wrui } }
    pub fn new_default() -> Ui { unsafe { Ui { wrui: wrui_get() } } }
    pub fn api_version(&self) -> u64 {
        unsafe {
            (*self.wrui).api_version
        }
    }

    pub fn application_create(&self) -> Application {
        unsafe {
            Application {
                funcs: (*self.wrui).application_funcs,
                obj: ((*self.wrui).application_create)()
           }
        }
    }

    pub fn window_create(&self) -> Window {
        unsafe {
            Window {
                funcs: (*self.wrui).window_funcs,
                obj: ((*self.wrui).window_create)()
           }
        }
    }

    pub fn push_button_create(&self) -> PushButton {
        unsafe {
            PushButton {
                funcs: (*self.wrui).push_button_funcs,
                obj: ((*self.wrui).push_button_create)()
           }
        }
    }

    pub fn main_window_create(&self) -> MainWindow {
        unsafe {
            MainWindow {
                funcs: (*self.wrui).main_window_funcs,
                obj: ((*self.wrui).main_window_create)()
           }
        }
    }

    pub fn dock_widget_create(&self) -> DockWidget {
        unsafe {
            DockWidget {
                funcs: (*self.wrui).dock_widget_funcs,
                obj: ((*self.wrui).dock_widget_create)()
           }
        }
    }

}
