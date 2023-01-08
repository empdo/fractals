use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
use std::time::Duration;
use stopwatch::Stopwatch;

use coolor;


pub fn draw_julia(
    canvas: &mut WindowCanvas,
    width: u32,
    height: u32,
    interval: f32,
    max_it: i32,
) {

    let cx = -0.8 + (0.01 * interval as f32);
    let cy = 0.26015 + (0.01 * interval  as f32);
    let rx = 3.5;
    let ry = 2.5;

    for x in 0..width {
        for y in 0..height{
            let mut zx = ((x as f32 / width as f32) * rx) -1.7;
            let mut zy = ((y as f32 / height as f32) * ry) - 1.2;
            
            let mut n: f64 = 0.0;
            while zx * zx + zy * zy < 4.0 && n < max_it as f64{
                let _zx = zx * zx - zy * zy;
                zy = 2.0 * zx * zy + cy;
                zx = _zx + cx;

                n += 1.0;
            }

            n = n / max_it as f64;

            let color = coolor::Hsl::new(286.0, 1.0, n as f32).to_rgb();
            canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
            canvas.fill_rect(Rect::new(x as i32,y as i32,1,1));


        }
    }
}

