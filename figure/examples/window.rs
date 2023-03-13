use figure::{Color, Rectangle, Window};

fn main() {
    let mut window = Window::new("Hello Figure!",1280,720);
    let rect = Rectangle::new(Color::Aqua,Color::Aqua,10,10,200,200);
    window.add(&rect);
    window.run();
}