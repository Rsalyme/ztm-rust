// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let mrp = 7;
    match mrp {
        3 => println!("Here's 3"),
        11 => println!("Here's 11"),
        12 => println!("Here's 12"),
        _ => println!("Here's {:?}",mrp),

    }
}
