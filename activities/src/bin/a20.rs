// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::str::FromStr;
use std::io;
#[derive(Debug)]
enum MachineState {
    OFF,
    SLEEP,
    REBOOT,
    SHUTDOWN,
    HIBERNATE
}


impl FromStr for MachineState {

    type Err = ();

    fn from_str(input: &str) -> Result<MachineState, Self::Err> {
        let binding = input.to_uppercase();
        let upper_input= binding.as_str();
        match &upper_input {
            &"OFF"  => {
                println!("Shutting Off");
                Ok(MachineState::OFF)
            },
            &"SLEEP"  => {
                println!("Going to Sleep");
                Ok(MachineState::SLEEP)
            },
            &"REBOOT" => 
            {
                println!("Rebooting....");
                Ok(MachineState::REBOOT)
            
            },
            &"SHUTDOWN" => {
                println!("Shutting Down");
                Ok(MachineState::SHUTDOWN)
            },            
            &"HIBERNATE" => {
                println!("Setting Hibernation Mode");
                Ok(MachineState::HIBERNATE)
            },
            _      => {
                println!(" I got nothing, Man");
                
                Err(())
            }
        }
    }
}

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned().to_uppercase())
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;
    while times_input < 1 {
        match read_input() {
            Ok(words) => {
                MachineState::from_str(words.as_str());
                all_input.push(words);
                times_input +=1;
            },
            Err(e) => println!("error {:?}",e)
        }
    }
}
