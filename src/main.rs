extern crate lodepng;
extern crate num;

use std::comm;
use types::{Point, Pixel, ImageIter};
use num::Complex;
mod types;

static h : u32 = 512;
static w : u32 = 1024;
static aspect : f32 = w as f32 / h as f32;
static pixel_count : uint = (w*h) as uint;

fn main() {
    let num_threads = std::rt::default_sched_threads();
    println!("Working on {} threads.", num_threads);
    let mut data = [0u8, ..pixel_count*3u];
    let (tx, rx) = comm::channel();
    let mut points = Vec::with_capacity(pixel_count);

    for x in range(0, w) {
        for y in range(0, h) {
            points.push(Point {x: x, y: y});
        }
    }

    let mut jobs = ImageIter::for_image_dimensions(w, h);
    for _ in range(0, num_threads) {
        let proc_tx = tx.clone();
        let job = jobs.next();
        match job {
            None => break,
            Some(rect) => {
                spawn(proc() {
                    println!("Started a thread!");
                    let num_pixels = rect.width as uint * rect.height as uint;
                    let mut pixels = Vec::with_capacity(num_pixels);
                    for point in rect.iter() {
                        pixels.push(generate_pixel(point));
                    }
                    proc_tx.send((rect, pixels));
                    println!("One completed!");
                });
            }
        }
    }
    let mut counter = num_threads;
    loop {
        let (rect, pixels) = rx.recv();
        for (point, pixel) in rect.iter().zip(pixels.iter()) {
            let index : uint = ((point.x + point.y * w) * 3) as uint;
            data[index + 0] = pixel.r;
            data[index + 1] = pixel.g;
            data[index + 2] = pixel.b;
        }
        let proc_tx = tx.clone();
        let job = jobs.next();
        match job {
            None => counter -= 1,
            Some(rect) => {
                spawn(proc() {
                    println!("Started a thread!");
                    let num_pixels = rect.width as uint * rect.height as uint;
                    let mut pixels = Vec::with_capacity(num_pixels);
                    for point in rect.iter() {
                        pixels.push(generate_pixel(point));
                    }
                    proc_tx.send((rect, pixels));
                    println!("One completed!");
                });
            }
        }
        if counter == 0 { break };
    }
        
    let path = &Path::new("test.png");
    match lodepng::encode_file(path, data, w, h, lodepng::LCT_RGB, 8) {
        Err(e) => fail!("Error writing: {}", e),
        Ok(_)  => (),
    }
}

fn pixel_mapping(point: Point) -> (f32, f32) {
    let mut x : f32 = point.x as f32;
    let mut y : f32 = point.y as f32;
    x *= 2.0; y *= 2.0;
    x -= w as f32; y -= h as f32;
    x /= w as f32; y /= w as f32;
    x *= aspect; y *= aspect;
    (x, y)
}

fn generate_pixel(point: Point) -> Pixel {
    let (x, y) = pixel_mapping(point);
    let c = Complex::new(x, y);
    let mut z = c.clone();
    let mut counter = 0;
    for i in range(0, 64) {
        counter = i;
        if z.norm() > 2.0 {
            break;
        }
        z = z*z + c;
    }
    
    let col : u8 = counter * 4;
    Pixel {r: col,
           g: col,
           b: col}
}
