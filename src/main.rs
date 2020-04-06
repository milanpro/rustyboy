mod gb_emulator;
use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let surface = Surface::create(&window);
    let size = window.inner_size();
    let width = size.width;
    let height = size.height;
    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels = Pixels::new(width, height, surface_texture).unwrap();

    loop {
        pixels.render();
    }

    gb_emulator::start_emulation();
}
