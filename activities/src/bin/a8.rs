// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum drinks {
    Pepsi,
    Tequila
}
struct DrinkData{
    name: drinks,
    fluidOunces: f64,
}
fn drinkInfo
fn main() {
     let thisdrink = DrinkData {
        name: drinks::Tequila,
        fluidOunces: 12.5,
    };
    match thisdrink.name{
        drinks::Pepsi => println!("its pepsi"),
        drinks::Tequila => println!("its tequila {:?}", thisdrink.fluidOunces)
    };
}
