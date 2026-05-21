#[path = "../creational_patterns/prototype/prototype.rs"]
mod prototype;

use prototype::{
    Circle,
    Rectangle,
    ShapeCache,
};

#[test]
fn test_circle_clone() {
    let original = ShapeCache::get_circle_prototype();

    let clone: Circle = original.clone();

    assert_eq!(original.radius, clone.radius);
}

#[test]
fn test_rectangle_clone() {
    let original = ShapeCache::get_rectangle_prototype();

    let clone: Rectangle = original.clone();

    assert_eq!(original.width, clone.width);
    assert_eq!(original.height, clone.height);
}
