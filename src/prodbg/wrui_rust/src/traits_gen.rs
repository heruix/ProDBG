use ::ffi_gen;

pub trait Widget {
    fn set_size(&self, width: i32, height: i32);
    fn get_obj(&self) -> *const ffi_gen::GUWidget;
}

