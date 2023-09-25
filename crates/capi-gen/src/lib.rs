mod capi_gen;
pub mod owned_slice;

pub use capi_gen_macro::capi_impl;
pub use capi_gen::*;


#[inline]
pub fn import_pointer<T>(value: *mut T) -> Box<T> {
    unsafe {
        Box::from_raw(value)
    }
}

#[inline]
pub unsafe fn export_pointer<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}
