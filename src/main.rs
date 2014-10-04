extern crate lodepng;

struct Pixel {r: u8, g: u8, b: u8}

static h : u32 = 10;
static w : u32 = 10;
static pixel_count : uint = (w*h) as uint;

fn main() {
    let path = &Path::new("test.png");

    let pixel_data = [Pixel {r: 0, g: 0, b: 0}, ..pixel_count];
    let mut data = [0u8, ..pixel_count*3u];
    assert!(pixel_data.len()*3 == data.len());
    for (i, p) in pixel_data.iter().enumerate() {
        data[i*3 + 0] = p.r;
        data[i*3 + 1] = p.g;
        data[i*3 + 2] = p.b;
    }
    match lodepng::encode_file(path, data, w, h, lodepng::LCT_RGB, 8) {
        Err(e) => fail!("Error writing: {}", e),
        Ok(_)  => (),
    }
}
