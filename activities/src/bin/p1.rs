// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::collections::HashMap;
use std::str::FromStr;
use std::io;
use std::fmt::Display;
#[derive(Hash, Eq, PartialEq, Debug)]
struct Bill {
    name: String,
    amount: i32,
}

impl Bill {
    /// Creates a new Viking.
    fn new(_name: &str, _amount: i32) -> Bill {
        Bill { name: _name.to_string(), amount: _amount }
    }
}

// Use a HashMap to store the vikings' health points.
fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned().to_uppercase())
}

fn find_bill(name: &str, bills:HashMap<String, Bill>){
    println!("{:?}",bills.get(name));
}
fn get_all_bills(bills:HashMap<String, Bill>){
    for bill in bills.values() {
        println!("{:?}",bill);
    }
}
fn remove_bill(name: &str,mut bills:HashMap<String, Bill>){
    bills.remove(name);
    for bill in bills.values() {
        println!("{:?}",bill);
    }
}

enum Menu {
    ADD,
    VIEW,
    REMOVE,
    EDIT
}

impl FromStr for Menu {
    type Err = ();

    fn from_str(input: &str) -> Result<Menu, Self::Err> {
        let binding = input.to_uppercase();
        let upper_input= binding.as_str();
        match &upper_input {
            &"ADD"  => {
                println!("Add a Bill");
                Ok(Menu::ADD)
            },
            &"VIEW"  => {
                println!("View all Bills");
                Ok(Menu::VIEW)
            },
            &"REMOVE" => 
            {
                println!("Remove a Bill");
                Ok(Menu::REMOVE)
            
            },
            &"EDIT" => {
                println!("Edit a Bill");
                Ok(Menu::EDIT)
            },
            _      => {
                println!("Unable to find that menu option.");
                
                Err(())
            }
        }
    }
}

fn main() {

    let mut bills:HashMap<String, Bill> = HashMap::new();
    let mut all_input = vec![];
    let mut times_input = 0;


    while times_input < 1 {
        match read_input() {
            Ok(words) => {
                Menu::from_str(words.as_str());
                all_input.push(words);
                times_input +=1;
            },
            Err(e) => println!("error {:?}",e)
        }
    }
}

