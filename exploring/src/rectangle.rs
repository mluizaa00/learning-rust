#[derive(Clone, Copy)]
pub struct Rectangle {
    height: f32,
    width: f32
}

pub fn create(height: f32, width: f32) {
    let rectangle = Rectangle { height, width };

    println!("A new rectangle has been created with the following information: ");
    println!("Height: {}", rectangle.height);
    println!("Width: {}", rectangle.width);
    area(rectangle);
}

pub fn area(rectangle: Rectangle) {
    let area = rectangle.height * rectangle.width;
    println!("Area: {}", area)
}