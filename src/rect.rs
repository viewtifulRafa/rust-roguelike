
/// A Room is just a rectangle
pub struct Rectangle {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl Rectangle {

    pub fn new (x: i32, y: i32, width:i32, height: i32) -> Rectangle {
        Rectangle {
            x0: x,
            y0: y,
            x1: x+width,
            y1: y+height
        }
    }

    pub fn intersects (&self, other: &Rectangle) -> bool {
        self.x0 <= other.x1 &&
        self.x1 >= other.x0 &&
        self.y0 <= other.y1 &&
        self.y1 >= other.y0
    }

    pub fn center (&self) -> (i32, i32) {
        ((self.x0 + self.x1)/2, (self.y0 + self.y1)/2) 
    }

}