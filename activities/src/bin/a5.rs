// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut l = 0;
    while l <= 3{
        l = l+1;
        println!("Number: {:?} ",l);
    }
    let mut i = 0;
    loop {
        println!("i is {}", i);
        if i >= 4 {
            break;
        }
        i = i+1;
    }
}
