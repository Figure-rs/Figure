use azusa::{Azusa, Color, FontInfo, ImageSurface, ImageType, UString};
use azusa::window::WindowSurface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

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

    pub fn set_title(&self,title: &str) {
        self.window.set_title(title);
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
                    self.azusa.set_source_color(Color::White);
                    self.azusa.clear();
                    self.azusa.draw(&mut self.surface);
                }
                _ => (),
            }
        });
    }
}