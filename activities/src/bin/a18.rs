// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase


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
