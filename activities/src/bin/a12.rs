// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
struct Box{
    dimensions: (i32,i32,i32),
    weight: f32,
    color: Color
}

impl Box{
    fn new(dimensions: (i32,i32,i32),weight: f32, color: Color) -> Self {
        return Self {
            dimensions,
            weight,
            color
        }
    }
    fn printChar(currentBox: Box){
        println!("{:?},{:?},{:?}",currentBox.dimensions,currentBox.weight,currentBox.color);
    }
}
#[derive(Debug)]
enum Color{
    Red,
    Yellow,
    Blue
}

fn main() {
    let shippingBox = Box::new((32,21,15),32.5,Color::Blue);

    Box::printChar(shippingBox);
}
