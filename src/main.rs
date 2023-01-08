extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
use std::time::Duration;

use stopwatch::Stopwatch;

use coolor;

pub mod mandel;
pub mod julia;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = 1200;
    let height = 900;

    let window = video_subsystem
        .window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut x_offset = 2.0;
    let mut y_offset = 0.7;
    let mut max_it = 100;
    let mut value = 2.0;
    let mut interval = 0.1;
    let mut zoom_step = 0;

    let mut time_step = 1;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    y_offset += interval;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    y_offset -= interval;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    x_offset -= interval;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    x_offset += interval;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    zoom_step += 1;
                    let _value = f32::powf(0.9, zoom_step as f32) * 0.2;

                    if (_value < 0.01) {
                        value -= 0.01;
                    } else {
                        value -= _value;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    zoom_step -= 1;
                    value += f32::powf(0.9, zoom_step as f32) * 0.2;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

       // mandel::draw_mandel(
       //     &mut canvas,
       //     width,
       //     height,
       //     x_offset,
       //     y_offset,
       //     max_it,
       //     value as f64,
       // );
        
        julia::draw_julia(&mut canvas, width, height, interval as f32, max_it);

        interval += 1.0;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1));
    }
}
