use std::ffi::{c_char, CStr};
use std::ops::Deref;
use std::ptr::{addr_of, addr_of_mut};
use figure::{Color, Rectangle, Widget, Window};

#[repr(C)]
pub struct FWidget(Box<dyn Widget>);

#[repr(C)]
pub enum CColor {
    White,
    Olive,
    Yellow,
    Fuchsia,
    Silver,
    Aqua,
    Lime,
    Red,
    Gray,
    Blue,
    Green,
    Purple,
    Black,
    Navy,
    Teal,
    Maroon
}

impl Into<Color> for CColor {
    fn into(self) -> Color {
        match self {
            CColor::White => Color::White,
            CColor::Olive => Color::Olive,
            CColor::Yellow => Color::Yellow,
            CColor::Fuchsia => Color::Fuchsia,
            CColor::Silver => Color::Silver,
            CColor::Aqua => Color::Aqua,
            CColor::Lime => Color::Lime,
            CColor::Red => Color::Red,
            CColor::Gray => Color::Gray,
            CColor::Blue => Color::Blue,
            CColor::Green => Color::Green,
            CColor::Purple => Color::Purple,
            CColor::Black => Color::Black,
            CColor::Navy => Color::Navy,
            CColor::Teal => Color::Teal,
            CColor::Maroon => Color::Maroon
        }
    }
}

#[no_mangle]
pub extern "C" fn figure_create_window() -> *mut Window {
    let window = Window::new("",100,100);
    let box_win = Box::new(window);
    Box::into_raw(box_win)
}

#[no_mangle]
pub extern "C" fn figure_set_window_title(window: *mut Window,title: *const c_char) {
    unsafe {
        let window = &*window;
        window.set_title(CStr::from_ptr(title).to_str().unwrap());
    }
}

#[no_mangle]
pub extern "C" fn figure_set_window_size(window: *mut Window,width: u32,height: u32) {
    unsafe {
        let window = &*window;
        window.set_size(width,height);
    }
}

#[no_mangle]
pub extern "C" fn figure_run_window(window: *mut Window) {
    unsafe {
        let window = Box::from_raw(window);
        window.run();
    }
}

#[no_mangle]
pub extern "C" fn figure_add(window: *mut Window,widget: *mut FWidget) {
    unsafe {
        let mut window = &mut *window;
        let widget = &*widget;
        let widget = widget.0.deref();
        window.add_trait(widget);
    }
}


#[no_mangle]
pub extern "C" fn figure_new_rectangle(color: CColor, border_color: CColor,x:u32,y:u32,width:u32,height:u32) -> *mut FWidget {
    let rectangle = Rectangle::new(color.into(),border_color.into(),x,y,width,height);
    let rectangle = Box::new(FWidget(Box::new(rectangle)));
    Box::into_raw(rectangle)
}

#[no_mangle]
pub extern "C" fn figure_new_label(background_color: CColor,foreground_color: CColor,text: *const c_char,x:u32,y:u32,width:u32,height:u32,px:u32) -> *mut FWidget {
    let label = Label::new(background_color.into(),foreground_color.into(),CStr::from_ptr(text).to_str().unwrap(),x,y,width,height,px);
    let label = Box::new(label);
    Box::into_raw(label)
}

#[no_mangle]
pub extern "C" fn figure_set_widget_background_color(widget: *mut FWidget,color: CColor) {
    unsafe {
        let widget = &*widget;
        widget.set_background_color(color.into());
    }
}