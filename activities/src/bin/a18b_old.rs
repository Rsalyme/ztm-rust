// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this


enum EmployeeType {
    MaintenanceCrew,
    Marketing,
    Managers,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech

}

struct Employee{
    current_employed: bool,
    employee_type: EmployeeType,
    name: String
}
fn grant_access(worker: &Employee) -> Result<bool,String>{
    if (worker.current_employed) {
        match worker.employee_type {
            EmployeeType::MaintenanceCrew => Ok(true),
            EmployeeType::Marketing => Ok(true),
            EmployeeType::Managers => Ok(true),
            _ =>  Err("Access Denied".to_owned()),
            
        }
    } else {
        Err("Access Denied".to_owned())
    }

}


fn main() {
    let will = Employee{ current_employed: true, employee_type: EmployeeType::Managers, name: "Will Smith".to_owned() };
    let jack = Employee{ current_employed: true, employee_type: EmployeeType::LineSupervisor, name: "Carlton Banks".to_owned() };

    let is_access_granted: Result<bool, String> = grant_access(&jack);
    match is_access_granted{
        Ok(_) => println!("Access Granted"),
        Err(_) => println!( "Nah bro ")
    };
}
