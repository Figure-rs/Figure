use azusa::Azusa;
use azusa::window::WindowSurface;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub type Color = azusa::Color;

pub trait Widget {
    fn reserve_drawing(&self,azusa: &mut Azusa);
}



pub struct Rectangle {
    color: Color,
    border_color: Color,
    x:u32,
    y:u32,
    width:u32,
    height:u32
}

impl Rectangle {
    pub fn new(color: Color,
               border_color: Color,
               x:u32,
               y:u32,
               width:u32,
               height:u32) -> Self {
        Self {
            color,
            border_color,
            x,
            y,
            width,
            height,
        }
    }
}

impl Widget for Rectangle {
    fn reserve_drawing(&self,azusa: &mut Azusa) {
        azusa.set_source_color(self.color);
        azusa.set_border_color(self.border_color);
        azusa.move_to(self.x,self.y);
        azusa.fill_rectangle(self.width,self.height);
    }
}

#[repr(C)]
pub struct Window {
    event_loop: Option<EventLoop<()>>,
    window: winit::window::Window,

    surface: WindowSurface,
    azusa: Azusa
}

impl Window {
    pub fn new(title: &str,width:u32,height:u32) -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width,height))
            .build(&event_loop)
            .unwrap();

        let surface = WindowSurface::new(&window).unwrap();
        let azusa = Azusa::new();

        Self {
            event_loop:Some(event_loop),
            window,
            surface,
            azusa
        }
    }

    pub fn add<T:Widget>(&mut self,widget: &T) {
        widget.reserve_drawing(&mut self.azusa);
    }

    pub fn add_trait(&mut self,widget: &dyn Widget) {
        widget.reserve_drawing(&mut self.azusa);
    }

    pub fn set_title(&self,title: &str) {
        self.window.set_title(title);
    }

    pub fn set_size(&self,width:u32,height:u32) {
        self.window.set_inner_size(LogicalSize::new(width,height));
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => control_flow.set_exit(),
                Event::MainEventsCleared => {
                    self.azusa.draw(&mut self.surface);
                }
                _ => (),
            }
        });
    }
}