// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
#[derive(Debug)]
enum Ticket {
    Backstage{
        name: String,
        price: i32
    },
    VIP{
        name: String,
        price: i32
    },
    Standard{
        price: i32
    }
}


fn main() {
    let smartpass = Ticket::Backstage{name: r#"Bob Johnson"#.to_owned(), price:4000};
    let quickpass = Ticket::Standard{ price:150};
    let fullpass = Ticket::VIP{name: "Mike Saint Layrent".to_owned(), price:2500};
    let tickets: Vec<Ticket> = vec![
        smartpass,
        quickpass,
        fullpass
    ];

for t in tickets {
    match t {
        Ticket::Backstage { name, price } => println!("Ticket Info: {:?} {:?} ",name, price),
        Ticket::VIP { name, price } => println!("Ticket Info: {:?} {:?}",name,price),
        Ticket::Standard { price } => println!("Ticket Info: {:?} ",price),
    }
};
    
}
