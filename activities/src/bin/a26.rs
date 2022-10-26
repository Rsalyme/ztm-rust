// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats
use chrono::prelude::*;
use chrono::offset::LocalResult;

fn main() {
    let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now();

    println!("Current Time UTC: {:?}", utc);
    println!("Current Time Local: {:?}", local.format("%d/%m/%Y %H:%M").to_string());
}
