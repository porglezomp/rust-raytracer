#![feature(globs)]

extern crate lodepng;
extern crate cgmath;

use std::comm;
use cgmath::*;
use image_types::{ScreenPoint, Pixel, Rect, ImageIter};
use scene::Scene;

mod image_types;
mod scene;

static h : u32 = 512;
static w : u32 = 1024;
static aspect : f32 = w as f32 / h as f32;
static pixel_count : uint = (w*h) as uint;

fn main() {
    let filename = "scene.json";
    let scene = scene::build_scene(filename.as_slice());
    let num_threads = std::rt::default_sched_threads();
    println!("Working on {} threads.", num_threads);
    let mut data = [0u8, ..pixel_count*3u];
    let (tx, rx) = comm::channel();
    let mut points = Vec::with_capacity(pixel_count);

    for x in range(0, w) {
        for y in range(0, h) {
            points.push(ScreenPoint {x: x, y: y});
        }
    }

    let mut jobs = ImageIter::for_image_dimensions(w, h);
    for _ in range(0, num_threads) {
        let job = jobs.next();
        match job {
            None => break,
            Some(rect) => new_worker(&tx, rect, scene.clone())
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
        let job = jobs.next();
        match job {
            None => counter -= 1,
            Some(rect) => new_worker(&tx, rect, scene.clone())
        }
        if counter == 0 { break };
    }
        
    let path = &Path::new("test.png");
    match lodepng::encode_file(path, data, w, h, lodepng::LCT_RGB, 8) {
        Err(e) => fail!("Error writing: {}", e),
        Ok(_)  => (),
    }
}

fn new_worker(tx: &Sender<(Rect, Vec<Pixel>)>, rect: Rect, scene: Scene) {
    let proc_tx = tx.clone();
    spawn(proc() {
        let num_pixels = rect.width as uint * rect.height as uint;
        let mut pixels = Vec::with_capacity(num_pixels);
        for point in rect.iter() {
            pixels.push(generate_pixel(point, &scene));
        }
        proc_tx.send((rect, pixels));
    });
}

fn pixel_mapping(point: ScreenPoint) -> (f32, f32) {
    let mut x : f32 = point.x as f32;
    let mut y : f32 = point.y as f32;
    x *= 2.0; y *= 2.0;
    x -= w as f32; y -= h as f32;
    x /= w as f32; y /= w as f32;
    x *= aspect; y *= aspect;
    (x, -y)
}

static camera_pos : Point3<f32> = Point3 {x: 0.0, y: 0.0, z: 0.0};
fn generate_pixel(point: ScreenPoint, scene: &Scene) -> Pixel {
    let (x, y) = pixel_mapping(point);
    let view_direction = Vector3::new(x, y, 1.0f32).normalize();
    let view_ray = Ray::new(camera_pos, view_direction);
    let c = scene.trace_ray(&view_ray);
    Pixel {r: (c.r * 255.0) as u8,
           g: (c.g * 255.0) as u8,
           b: (c.b * 255.0) as u8}
}
