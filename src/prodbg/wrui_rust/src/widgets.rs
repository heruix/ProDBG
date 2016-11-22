pub Application {
    funcs: *const GUApplicationFuncs,
    obj: *const GUApplication,
}

impl Application {
    fn run(&mut self) -> i32,
}

pub Window {
    funcs: *const GUWindowFuncs,
    obj: *const GUWindow,
}

impl Window {
    fn set_title(&mut self),
}

pub PushButton {
    funcs: *const GUPushButtonFuncs,
    obj: *const GUPushButton,
}

impl PushButton {
    fn set_default(&mut self, state: i32),
}

pub MainWindow {
    funcs: *const GUMainWindowFuncs,
    obj: *const GUMainWindow,
}

impl MainWindow {
    fn add_dock_widget(&mut self, area: u32, widget: *const GUDockWidget),
}

pub DockWidget {
    funcs: *const GUDockWidgetFuncs,
    obj: *const GUDockWidget,
}

impl DockWidget {
    fn is_floating(&mut self) -> bool,
    fn set_floating(&mut self, floating: bool),
    fn set_widget(&mut self, widget: *const GUWidget),
    fn widget(&mut self) -> *const GUWidget,
}

