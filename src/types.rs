#[deriving(Clone, Show)]
pub struct Point {
    pub x: u32,
    pub y: u32
}
#[deriving(Clone, Show)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
#[deriving(Clone, Show)]
pub struct Rect {
    pub origin: Point,
    pub width: u32,
    pub height: u32
}
pub struct RectIter {
    pub rect: Rect,
    pub position: Point
}

impl Rect {
    pub fn iter(&self) -> RectIter {
        RectIter {rect: *self, 
                  position: Point {x: 0, y: 0}}
    }
}
impl Add<Point, Point> for Point {
    fn add(&self, rhs: &Point) -> Point {
        Point {x: self.x + rhs.x,
               y: self.y + rhs.y}
    }
}

impl Iterator<Point> for RectIter {
    fn next(&mut self) -> Option<Point> {
        if self.position.y >= self.rect.height {
            return None;
        }
        let value = self.position;
        self.position.x += 1;
        if self.position.x >= self.rect.width {
            self.position.x = 0;
            self.position.y += 1;
        }
        Some(self.rect.origin + value)
    }
}

#[deriving(Show)]
pub struct ImageIter {
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
    pub fn for_image_dimensions(w: u32, h: u32) -> ImageIter {
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
