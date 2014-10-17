#![feature(globs)]

extern crate lodepng;
extern crate cgmath;
extern crate serialize;

use std::sync::Arc;
use std::comm;
use cgmath::*;
use image_types::{ScreenPoint, Pixel, Rect, ImageIter};
use scene::Scene;

mod image_types;
mod scene;
mod parse_scene;

const W : u32 = 640;
const H : u32 = 480;
const ASPECT : f32 = W as f32 / H as f32;
const PIXEL_COUNT : uint = (W*H) as uint;

fn main() {
    let filename = "scene.json";
    let scene = Arc::new(scene::build_scene(filename.as_slice()));
    let num_threads = std::rt::default_sched_threads();
    println!("Working on {} threads.", num_threads);
    let mut data = [0u8, ..PIXEL_COUNT*3u];
    let (tx, rx) = comm::channel();
    let mut points = Vec::with_capacity(PIXEL_COUNT);

    for x in range(0, W) {
        for y in range(0, H) {
            points.push(ScreenPoint {x: x, y: y});
        }
    }

    let mut counter = 0u;
    let mut jobs = ImageIter::for_image_dimensions(W, H);
    for _ in range(0, num_threads) {
        let job = jobs.next();
        match job {
            None => break,
            Some(rect) => {
                counter += 1;
                new_worker(&tx, rect, scene.clone())
            }
        }
    }
    loop {
        let (rect, pixels) = rx.recv();
        for (point, pixel) in rect.iter().zip(pixels.iter()) {
            let index : uint = ((point.x + point.y * W) * 3) as uint;
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
    match lodepng::encode_file(path, data, W, H, lodepng::LCT_RGB, 8) {
        Err(e) => fail!("Error writing: {}", e),
        Ok(_)  => (),
    }
}

fn new_worker(tx: &Sender<(Rect, Vec<Pixel>)>, rect: Rect, scene: Arc<Scene>) {
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
    x -= W as f32; y -= H as f32;
    x /= W as f32; y /= W as f32;
    x *= ASPECT; y *= ASPECT;
    (x, -y)
}

const CAMERA_POS : Point3<f32> = Point3 {x: 0.0, y: -2.0, z: 0.0};
fn generate_pixel(point: ScreenPoint, scene: &Arc<Scene>) -> Pixel {
    let (x, y) = pixel_mapping(point);
    let view_direction = Vector3::new(x, 1.0f32, y).normalize();
    let view_ray = Ray::new(CAMERA_POS, view_direction);
    let c = scene.trace_ray(&view_ray, 0).saturate();
    Pixel {r: (c.r * 255.0) as u8,
           g: (c.g * 255.0) as u8,
           b: (c.b * 255.0) as u8}
}
