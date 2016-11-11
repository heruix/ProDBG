mod ffi;

pub struct Ui {
	wrui: *const ffi::Wrui;
}


impl Ui {
	pub fn new(*const ffi::Wrui) -> Ui {
		Ui {
			wrui: ffi::Wrui
		}

	pub fn new_default() -> Ui {
		Ui {
			wrui: ffi::wrui_get(),
		}
	}
}

