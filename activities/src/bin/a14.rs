// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
use std::slice;
use std::str;

struct Person {
    name:  &'static str,
    age: i32,
    color:  &'static str
}
fn main() {
    let people: Vec<Person> = vec![
        Person{name:"Jon",age:19,color:"blue"},
        Person{name:"Sam",age:7,color:"red"},
        Person{name:"Tim",age:9,color:"green"}];
    for p in people {
        if p.age < 10 {
            print_name_and_color(&p);
        }
    }
}

fn print_name_and_color(p:&Person) {
    println!("{} {}",p.name,p.color);
}
