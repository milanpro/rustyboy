mod gb_emulator;
use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use std::thread;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    thread::spawn(move || loop {
        let surface = Surface::create(&window);
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        let surface_texture = SurfaceTexture::new(width, height, surface);
        let mut pixels = Pixels::new(width, height, surface_texture).unwrap();

        loop {
            let frame = pixels.get_frame();
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                pixel[0] = (i as u32 % width) as u8;
                pixel[1] = (i as u32 % height) as u8;
                pixel[2] = (0xff) as u8;
                pixel[3] = 0xff;
            }

            pixels.render();
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });

    gb_emulator::start_emulation();
}
