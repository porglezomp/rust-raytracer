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
