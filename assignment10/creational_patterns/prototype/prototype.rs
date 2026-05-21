#[derive(Clone)]
pub struct Circle {
    pub radius: u32,
}

impl Circle {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }
}

#[derive(Clone)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub struct ShapeCache;

impl ShapeCache {
    pub fn get_circle_prototype() -> Circle {
        Circle::new(10)
    }

    pub fn get_rectangle_prototype() -> Rectangle {
        Rectangle::new(20, 15)
    }
}
