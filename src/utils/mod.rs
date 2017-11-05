pub mod image;
pub use self::image::*;

use std::os::raw::c_void;

pub fn vec_void_ptr<T>(vec: &Vec<T>) -> *const c_void {
    vec.as_ptr() as *const c_void
}