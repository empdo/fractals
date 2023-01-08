use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
use std::time::Duration;

use stopwatch::Stopwatch;

use coolor;


pub fn draw_julia (
    canvas: &mut WindowCanvas,
    width: u32,
    height: u32,
    x_offset: f64,
    y_offset: f64,
    max_it: i32,
    value: f64,
) {

    for x in 0..width {
        for y in 0..height {
            let mut zx = (((x as f64) / (width as f64)) * value) - x_offset;
            let mut zy = (((y as f64) / (height as f64)) * value) - y_offset;

            let cx = zx;
            let cy = zy;

            let mut n: f64 = 0.0;
            while n < max_it as f64 {
                let _zx = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = _zx;

                if (zx * zx + zy * zy) > 4.0 {
                    break;
                }
                n += 1.0;
            }

            n = n / max_it as f64;
            let mut color: coolor::Rgb;

            
            color = coolor::Hsl::new(286.0, 1.0, n as f32).to_rgb();
            canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
            canvas.fill_rect(Rect::new(x as i32, y as i32, 1, 1));
        }
    }
//    println!("Took {}ms to draw shape", sw.elapsed_ms());
//    sw.stop();
}
