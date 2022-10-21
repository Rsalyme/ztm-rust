// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct student_locker{
    name: String,
    locker_assignment:Option<i32>,
}

fn main() {
    let jackSparrow = student_locker{name: "Jack Sparrow".to_owned(),locker_assignment:None};
    let jodieSparrow = student_locker{name: "Jodie Sparrow".to_owned(),locker_assignment:Some(152)};
    let students = vec![jodieSparrow,jackSparrow];

    for student in students {
        println!("Student Locker Info: {:?} Locker Number: {:?}", student.name, student.locker_assignment);
    }
    
}
