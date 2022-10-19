// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
#[derive(Debug)]
enum Colors {
    Red,
    Blue,
    Green
}
fn printColor(color: Colors){
    match color {
        Colors::Red => println!("Red"),
        Colors::Blue => println!("Blue"),
        Colors::Green => println!("Green"),
    };
}
fn main() {

    printColor(Colors::Green);
}
