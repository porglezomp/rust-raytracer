extern crate lodepng;

use std::comm;
use types::{Point, Pixel, Rect};
mod types;

static h : u32 = 512;
static w : u32 = 512;
static pixel_count : uint = (w*h) as uint;

fn main() {
    let num_threads = std::rt::default_sched_threads();
    let mut data = [0u8, ..pixel_count*3u];
    let (tx, rx) = comm::channel();
    let mut points = Vec::with_capacity(pixel_count);

    for x in range(0, w) {
        for y in range(0, h) {
            points.push(Point {x: x, y: y});
        }
    }

    let mut jobs = ImageIter::new();
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

#[deriving(Show)]
struct ImageIter {
    x: u32,
    y: u32,
    num_tiles_x: u32,
    num_tiles_y: u32,
    tile_w: u32,
    tile_h: u32,
    end_row_w: u32,
    end_row_h: u32
}

impl ImageIter {
    fn new() -> ImageIter {
        let (tile_w, tile_h) = (128, 128);
        ImageIter {
            x: 0,
            y: 0,
            num_tiles_x: w/tile_w,
            num_tiles_y: h/tile_h,
            tile_w: tile_w,
            tile_h: tile_h,
            end_row_w: w % tile_w,
            end_row_h: h % tile_h
        }
    }
}

impl Iterator<Rect> for ImageIter {
    fn next(&mut self) -> Option<Rect> {
        // If we're on the border of the image, use the potentially
        // smaller dimensions in order to pad out the image size,
        // otherwise, use the default tile size.
        let use_w = if self.x == self.num_tiles_x {
                       self.end_row_w } else { self.tile_w };
        let use_h = if self.y == self.num_tiles_y {
                       self.end_row_h } else { self.tile_h };

        let current = Rect { origin: Point {x: self.x*self.tile_w,
                                            y: self.y*self.tile_h },
                             width: use_w,
                             height: use_h };
        if self.y > self.num_tiles_y {
            return None;
        }
        self.x += 1;
        if self.x > self.num_tiles_x {
            self.x = 0;
            self.y += 1;
        }

        // If the current tile has no width or height, go to the next one
        if current.width == 0 || current.height == 0 {
            self.next()
        } else {
            Some(current)
        }
    }
}

fn generate_pixel(point: Point) -> Pixel {
    Pixel {r: (point.x % 256) as u8,
           g: (point.y % 256) as u8,
           b: 128}
}
