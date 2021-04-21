#[derive(Clone, Copy)]
pub struct Rectangle {
    height: f32,
    width: f32
}

/*
Creates a new Rectangle with the 
Height and Width provided
 */
pub fn create(height: f32, width: f32) {
    if height == width {
        panic!("A rectangle can't have the same height and width!");
    }

    let rectangle = Rectangle { height, width };

    println!("A new rectangle has been created with the following information: ");
    println!("Height: {}", rectangle.height);
    println!("Width: {}", rectangle.width);
    area(rectangle);
    perimeter(rectangle);
    diagonal(rectangle);
}

/*
Returns the area of the Rectangle provided
Calc: height * width
 */
pub fn area(rectangle: Rectangle) {
    let area = rectangle.height * rectangle.width;
    println!("Area: {}", area)
}

/*
Returns the perimeter of the Rectangle provided
Calc: height * 2 + width * 2
 */
pub fn perimeter(rectangle: Rectangle) {
    let perimeter = (rectangle.height * 2.0) + (rectangle.width * 2.0);
    println!("Perimeter: {}", perimeter);
}

/*
Returns the diagonal of the Rectangle provided
Calc: square root of (height * height + width * width)
 */
pub fn diagonal(rectangle: Rectangle) {
    let diagonal = (rectangle.height * rectangle.height) + (rectangle.width * rectangle.width);
    println!("Diagonal: {}", diagonal.sqrt())
}
