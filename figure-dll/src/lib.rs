use std::ffi::{c_char, CStr, CString};
use std::ptr::addr_of;
use once_cell::sync::{Lazy, OnceCell};

pub type Int = i64;

#[no_mangle]
pub extern "C" fn create_figure_window(title:*const c_char,width: i32,height: i32) -> Box<*const Int> {
    unsafe {
        let window = figure::Window::new(CStr::from_ptr(title).to_str().unwrap(), width as u32, height as u32);
        println!("Create window");
        let x = Box::new(addr_of!(window) as *const Int);
        println!("Get pointer");
        x
    }
}

#[no_mangle]
pub extern "C" fn run_figure_window(window: *const Box<*const Int>) {
    unsafe {
        let window = window as *const Box<*const figure::Window>;
        (*window).read_volatile().run();
    }
}