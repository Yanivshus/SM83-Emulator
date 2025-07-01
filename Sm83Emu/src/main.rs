use crate::cpu::Cpu;
mod cartridge;
mod cpu;
mod gfx;
mod memory;

use anyhow::Error;
use pixels::{Pixels, SurfaceTexture};

use winit::{
    dpi::{LogicalSize, Pixel},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use winit::window::{WindowAttributes, WindowBuilder};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("GameBoy emulator")
        .with_inner_size(LogicalSize::new(256 as f64, 256 as f64))
        .build(&event_loop)
        .expect("Failed to create window");

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();
    let res = event_loop
        .run(|event, elwt| {
            match event {
                Event::AboutToWait => {
                    window.request_redraw();
                }

                // Called when it's time to draw
                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::RedrawRequested,
                } => {
                    draw(pixels.frame_mut());

                    if let Err(err) = pixels.render() {
                        eprintln!("Render error: {}", err);
                        elwt.exit();
                    }
                }

                // Handle window close
                Event::WindowEvent { event, .. } => {
                    if let WindowEvent::CloseRequested = event {
                        elwt.exit();
                    }
                }

                _ => {}
            }
        })
        .unwrap();

    //res.map_err(|e| Error::UserDefined(Box::new(e)))
    // Draw it to the `SurfaceTexture`
}

fn draw(frame: &mut [u8]) {
    println!("{}", frame.len() / 4);
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        pixel[0] = 0xFF; // R
        pixel[1] = 0x00; // G
        pixel[2] = 0x00; // B
        pixel[3] = 0x50; // A
    }
}

// color -> a,r,g,b
// struct Color(u8,u8,u8,u8);
// fn get_color(val: u8) -> Result<Color, Error> {
//     let res = match val {
//         0 =>,
//         1 =>,
//         2 =>,
//         3 =>,
//         _ => Err()

//     }
// }
