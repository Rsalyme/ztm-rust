/*
Topic: Testing

Requirements:
* Write tests for the existing program to ensure proper functionality.

Notes:
* Create at least two test cases for each function.
* Use `cargo test` to test the program.
* There are intentional bugs in the program that need to be fixed.
  * Check the documentation comments for the functions to
    determine how the they should operate.
*/

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}


#[cfg(test)]
mod test{

    use crate::*;

    #[test]
    fn clamp_test_1(){
        let n = 15;
        let upper = 36;
        let lower = 12;

        let result = clamp(n, lower, upper);
        assert_eq!(result,n, "The numbers don't match");
    }
    #[test]
    fn clamp_test_2(){
        let n = 48;
        let upper = 36;
        let lower = 12;

        let result = clamp(n, lower, upper);
        assert_eq!(result,upper, "The numbers don't match");
    }

    #[test]
    fn div1(){
        let n = 12;
        let q = 4;
        let result = div(n, q);
        assert_eq!(result,Some(3), "The numbers don't match");
    }
    #[test]
    fn div2(){
        let n = 1000;
        let q = 50;
        let result = div(n, q);
        assert_eq!(result,Some(20), "The numbers don't match");
    }
    #[test]
    fn concat1(){
        let n = "Hello";
        let q = " world";
        let result = concat(n, q);
        assert_eq!(result,"Hello world", "The outcomes don't match");
    }
    #[test]
    fn concat2(){
        let n = "Jack O'";
        let q = " Lantern";
        let result = concat(n, q);
        assert_eq!(result,"Jack O' Lantern".to_string(), "The outcomes don't match");
    }

}
