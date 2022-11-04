// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    fn add(a: i32,b: i32) -> i32{
        return a+b;
    }
    fn prlin(){
        println!("{:?}", add(3,2));
    }
    prlin();

}
