// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print
fn sendTuple(tup: (i32,i32,i32)) {
    let (x,y,z) = (tup.0,tup.1,tup.2);
    let mut sign;
    if y > 5 {
        sign="greater than 5";
    } else if y < 5 {
        sign ="less than 5";
    } else if y == 5 {
          sign ="equal to 5";
    } else {
        sign ="idk";
    }

    return (println!("{}",sign))
}
fn main() {
    let test = (2,5,9);
    sendTuple(test)

}
