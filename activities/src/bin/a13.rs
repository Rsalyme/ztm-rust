// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector
struct Number{
    digit: i32,
    name: String
}
fn main() {
    let digits: Vec<Number> = vec![
        Number{digit: 10, name:"Ten".to_string() },
        Number{digit: 20, name:"Twenty".to_string() },
        Number{digit: 30, name:"Thirty".to_string() },
        Number{digit: 40, name:"Forty".to_string()}
    ];
    for num in &digits {
        println!("{:?} {:?}", num.digit,num.name);
    }
    println!("number of elements = {:?}", digits.len());
}
