use crate::cpu::Cpu;
mod cartridge;
mod cpu;
mod gfx;
mod memory;

//use anyhow::{Error, Ok};
use pixels::{Pixels, SurfaceTexture};

use winit::{
    dpi::{LogicalSize, Pixel},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use rand::{ Rng};

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
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let mut rng = rand::thread_rng();
        let curr = rng.gen_range(0..4);
        (pixel[3] ,pixel[2] ,pixel[1] ,pixel[0]) = get_color(curr).unwrap()
    }
}

//Lightest Green: Hex #9bbc0f, RGB (155, 188, 15)
//Light Green: Hex #8bac0f, RGB (139, 172, 15)
//Dark Green: Hex #306230, RGB (48, 98, 48)
//Darkest Green: Hex #0f380f, RGB (15, 56, 15)
//color -> a,r,g,b
#[derive(Debug)]
struct ColorError;

fn get_color(val: u8) -> Result<(u8,u8,u8,u8), ColorError> {
    let res = match val {
        0 => return Ok((0xff, 155,188,15)), 
        1 => return Ok((0xff, 139, 172, 15)),
        2 => return Ok((0xff, 48,98,48)),
        3 => return Ok((0xff, 15,56,15)),
        _ => return Err(ColorError)
    };
}
