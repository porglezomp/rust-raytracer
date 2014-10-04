extern crate lodepng;

use std::comm;

#[deriving(Clone)]
struct Point {x: u32, y: u32}
#[deriving(Clone)]
struct Pixel {r: u8, g: u8, b: u8}

static h : u32 = 512;
static w : u32 = 512;
static pixel_count : uint = (w*h) as uint;

fn main() {
    let mut data = [0u8, ..pixel_count*3u];
    let (tx, rx) : (Sender<(Point, Pixel)>, Receiver<(Point, Pixel)>) = comm::channel();
    let mut points : Vec<Point> = Vec::with_capacity(pixel_count);

    for x in range(0, w) {
        for y in range(0, h) {
            points.push(Point {x: x, y: y});
        }
    }
    
    for p in points.iter() {
        let proc_tx = tx.clone();
        let point = p.clone();
        spawn(proc() {
            proc_tx.send((point, render(point)));
        });
    }
    
    for _ in range(0, pixel_count) {
        let (point, pixel) = rx.recv();
        let index : uint = ((point.x + point.y * w) * 3) as uint;
        data[index + 0] = pixel.r;
        data[index + 1] = pixel.g;
        data[index + 2] = pixel.b;
    }

    let path = &Path::new("test.png");
    match lodepng::encode_file(path, data, w, h, lodepng::LCT_RGB, 8) {
        Err(e) => fail!("Error writing: {}", e),
        Ok(_)  => (),
    }
}

fn render(point: Point) -> Pixel {
    Pixel {r: (point.x % 256) as u8,
           g: (point.y % 256) as u8,
           b: 128}
}
