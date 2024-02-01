mod mandelbrot;

use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use crate::mandelbrot::Mandelbrot;

const MAX_GEN_THREADS: u32 = 8;
const ZOOMING_RATE: f64 = 0.01;
const MOVING_RATE: f64 = 5.0;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to initialize EventLoop");
    let window = Rc::new(WindowBuilder::new().build(&event_loop).expect("Failed to initialize Window"));
    let context = Context::new(window.clone()).expect("Failed to initialize window context.");
    let mut surface = Surface::new(&context, window.clone()).expect("Failed to init surface.");

    let mut scale = 0.4;
    let mut x_additional_offset = 0.0;
    let mut y_additional_offset = 0.0;

    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => { window_target.exit(); }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event, .. }, .. } => {
                let is_pressed = event.state == ElementState::Pressed;
                let key_target = event.logical_key.to_text();

                if is_pressed && key_target.is_some() { match key_target.unwrap() {
                    "i" => { scale += ZOOMING_RATE; }                // ZOOM IN
                    "o" => { scale -= ZOOMING_RATE; }                // ZOOM OUT
                    "l" => { x_additional_offset += MOVING_RATE; }   // RIGHT
                    "r" => { x_additional_offset -= MOVING_RATE; }   // LEFT
                    "t" => { y_additional_offset += MOVING_RATE; }   // TOP
                    "b" => { y_additional_offset -= MOVING_RATE; }   // BOTTOM
                    _ => ()
                } }

                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let (width, height) = window.inner_size().into();

                surface.resize(NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap())
                    .expect("Failed to resize the surface.");

                let mut mandelbrot = Mandelbrot::init_with_offset_and_scale_for_coords(
                    (width, height), scale
                );
                mandelbrot.offset_x += x_additional_offset;
                mandelbrot.offset_y += y_additional_offset;

                // To guarantee quick rendering time, the Mandelbrot generation is shared between threads. Hence, a
                // static `MAX_GEN_THREADS` number of threads is created and sectors of the window are divided and
                // shared between every thread.
                // The window is divided horizontally, and to ensure maximum efficiency, generation results are shared
                // through a channel whose listener directly adds computed colors to the display buffer.

                let (tx, rx): (Sender<(u32, u32)>, Receiver<(u32, u32)>) = mpsc::channel();
                let thread_gen_width_res = width / MAX_GEN_THREADS;

                for thread in 0..MAX_GEN_THREADS {
                    let thread_tx = tx.clone();

                    thread::spawn(move || {
                        let width_offset = thread_gen_width_res * thread;

                        for x in width_offset..(width_offset + thread_gen_width_res) {
                            for y in 0..height {
                                let color = mandelbrot.get_point_color_at_coords(
                                    x.clone() as i32, y.clone() as i32
                                );
                                let pixel_number = y * width + x;
                                let bor_pixel = color[0] as u32 | (color[1] as u32) << 8 | (color[2] as u32) << 16;

                                thread_tx.send((bor_pixel, pixel_number)).unwrap();
                            }
                        }
                    });
                }

                let mut buffer = surface.buffer_mut().unwrap();
                for _ in 0..(width * height) {
                    let received_color = rx.recv().unwrap();
                    buffer[received_color.1 as usize] = received_color.0;
                }

                buffer.present().unwrap();

                println!("Rendered {} pixels with Mandelbrot.", width * height);
            }
            _ => ()
        }
    }).expect("Event loop listener failed.");
}
