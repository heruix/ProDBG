use std::ffi::CString;
use std::ptr;
use std::os::raw::{c_void};

mod ffi;

pub struct Ui {
	wrui: *const ffi::Wrui,
}


impl Ui {
	pub fn new(wrui: *const ffi::Wrui) -> Ui {
		Ui {
			wrui: wrui, 
		}
	}

	pub fn new_default() -> Ui {
	    unsafe {
            Ui {
                wrui: ffi::wrui_get(),
            }
        }
	}

	pub fn api_version(&self) -> u64 {
	    unsafe {
	        (*self.wrui).api_version
        }
    }

    pub fn application_create(&self) -> Application {
        unsafe {
            Application {
                funcs: (*self.wrui).application_funcs,
                app: ((*self.wrui).application_create)()
            }
        }
    }

    pub fn push_button_create(&self, name: &str, _parent: Option<&Widget>) -> PushButton {
        let name = CString::new(name).unwrap();

        /*
        let par_widget;
        if let Some(p) = parent {
            par_widget = p.widget;
        } else {
            par_widget = ptr::null();
        }
        */

        unsafe {
            PushButton {
                funcs: (*self.wrui).push_button_funcs,
                widget: ((*self.wrui).push_button_create)(name.as_ptr(), ptr::null())
            }
        }
    }

}

pub struct Widget {
    _widget: *mut ffi::GUWidget,
}

///
/// Application implementation
///

pub struct Application {
    funcs: *mut ffi::GUApplicationFuncs,
    app: *const ffi::GUApplication,
}

impl Application {
    pub fn run(&self) {
        unsafe {
            ((*self.funcs).run)(self.app as *mut c_void);
        }
    }
}

///
/// PushButton implementation
///

pub struct PushButton {
    funcs: *mut ffi::GUPushButtonFuncs,
    widget: *const ffi::GUPushButton,
}



