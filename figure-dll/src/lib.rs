use std::ffi::{c_char, CStr, CString};
use once_cell::sync::{Lazy, OnceCell};
use figure::Window;

pub static mut WINDOW: Lazy<OnceCell<Window>> = Lazy::new(|| {OnceCell::new()});

#[no_mangle]
pub extern "C" fn create_figure_window(title:*const c_char,width: i32,height: i32){
    unsafe {
        let win = Window::new(CStr::from_ptr(title).to_str().unwrap(), width as u32, height as u32);
        match WINDOW.set(win) {
            Ok(_) => {}
            Err(_) => {
                panic!("Window is already created");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn run_figure_window() {
    unsafe {
        let mut win= WINDOW.take().unwrap();
        win.run();
    }
}