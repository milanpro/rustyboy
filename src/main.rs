use core::panic;
use interpolation::Lerp;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use rustyboy::start_emulation;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let size = window.inner_size();
    let width = size.width;
    let height = size.height;
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = match Pixels::new(width, height, surface_texture) {
        Ok(pixels) => pixels,
        Err(error) => {
            error!("Creating pixels backend failed: {}", error);
            panic!();
        }
    };

    start_emulation();

    event_loop
        .run(move |event, elwt| {
            if let Event::WindowEvent {
                event: window_event,
                ..
            } = &event
            {
                if window_event == &WindowEvent::RedrawRequested {
                    let win_width = window.inner_size().width;
                    let frame = pixels.frame_mut();
                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let r: u8 =
                            0x0.lerp(&0xff, &((i as u32 % win_width) as f32 / win_width as f32));
                        pixel.copy_from_slice(&[r, 0, 0, 0xff]);
                    }
                    if pixels
                        .render()
                        .map_err(|e| error!("pixels.render() failed: {}", e))
                        .is_err()
                    {
                        elwt.exit();
                        return;
                    }
                }
            }

            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(KeyCode::Escape)
                    || input.close_requested()
                    || input.destroyed()
                {
                    elwt.exit();
                    return;
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    if let Err(err) = pixels
                        .resize_surface(size.width, size.height)
                        .and_then(|_| pixels.resize_buffer(size.width, size.height))
                    {
                        error!("pixels.resize_surface {:?}", err);
                        elwt.exit();
                        return;
                    }
                    window.request_redraw();
                }
            }
        })
        .unwrap();
}
