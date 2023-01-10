extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
use std::time::Duration;
use std::env;

use stopwatch::Stopwatch;

use coolor;

pub mod mandel;
pub mod julia;
pub mod ship;

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

    let args: Vec<String> = env::args().collect();

    let query = &args[1];

    println!("Searching for {}", query);
    let mut x_offset = 2.0;
    let mut y_offset = 0.7;
    let mut max_it = 100;
    let mut value = 2.0;
    let mut interval = 0.1;
    let mut zoom_step = 0;
    let mut zoom_interval = 1;


    if query == "ship" {
        zoom_interval = 2;
        
        x_offset = 2.0;
        y_offset = 0.3;
        value = 0.59199154;
    } 

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
                    println!("y_offset: {y_offset}");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    y_offset -= interval;
                    println!("y_offset: {y_offset}");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    x_offset -= interval;
                    println!("x_offset: {x_offset}");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    x_offset += interval;

                    println!("x_offset: {x_offset}");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    zoom_step += zoom_interval;
                    let _value = f32::powf(0.9, zoom_step as f32) * 0.2;

                    if _value < 0.01 {
                        value -= 0.01;
                    } else {
                        value -= _value;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    zoom_step -= zoom_interval;
                    value += f32::powf(0.9, zoom_step as f32) * 0.2;

                    println!("value: {value}");
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        //
                
        if query == "mandel" {
            mandel::draw_mandel(
                &mut canvas,
                width,
                height,
                x_offset,
                y_offset,
                max_it,
                value as f64,
            );
        } else if query == "julia" {
            julia::draw_julia(&mut canvas, width, height, time_step as f32, max_it);
        } else if query == "ship" {
            ship::draw_ship(
                &mut canvas,
                width,
                height,
                x_offset,
                y_offset,
                max_it,
                value as f64,
            );
        } 

        

        time_step += 1;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1));
    }
}
