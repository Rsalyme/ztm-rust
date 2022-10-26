// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message


struct customer{
    name: String,
    age: i32
}

fn is_age_restricted(cust:&customer ) -> Result<bool,String>{
    if 21 <= cust.age {
        Ok(true)
    } else {
        Err("this person's age information doesn't allow them to make this purchase".to_owned())
    }
}
fn main() {
    let timmy = customer{
        name: "Tim Tim".to_owned(),
        age: 21
    };
    let can_they_drink: Result<bool, String> = is_age_restricted(&timmy);

    match can_they_drink{
        Ok(_) => println!("{:?}", "They can drink"),
        Err(_) => println!("{:?}", "Nah bro ")
    };

}
