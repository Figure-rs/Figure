use std::ffi::{c_char, c_int, CStr, CString};
use std::ptr::addr_of;
use once_cell::sync::{Lazy, OnceCell};
use figure::Window;

#[no_mangle]
pub extern "C" fn create_figure_window(title:*const c_char,width: i32,height: i32) -> *mut Window {
    unsafe {
        let window = Window::new(CStr::from_ptr(title).to_str().unwrap(), width as u32, height as u32);
        let box_win = Box::new(window);
        Box::into_raw(box_win)
    }
}

#[no_mangle]
pub extern "C" fn set_figure_window_title(window: *mut Window,title: *const c_char) {
    unsafe {
        let window = &*window;
        window.set_title(CStr::from_ptr(title).to_str().unwrap());
    }
}

#[no_mangle]
pub extern "C" fn run_figure_window(window: *mut Window) {
    unsafe {
        let window = Box::from_raw(window);
        window.run();
    }
}