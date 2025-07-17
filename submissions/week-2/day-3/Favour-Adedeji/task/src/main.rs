mod lib;
use crate::lib::{can_access_building, Department, EmployeeDetails};

fn main() {
    let employee1 = EmployeeDetails {
        name: "Alice".to_string(),
        department: Department::MEDIA,
        is_terminated: false,
    };

    let employee2 = EmployeeDetails {
        name: "Bob".to_string(),
        department: Department::SOCIAL_MEDIA,
        is_terminated: false,
    };

    let employee3 = EmployeeDetails {
        name: "Rick".to_string(),
        department: Department::SOCIAL_MEDIA,
        is_terminated: true,
    };

    let res = can_access_building(&employee1).unwrap();
    println!("Access for {}: {}", employee1.name, res);

    // let res2 = can_access_building(&employee2)?
    // println!("Access for {}: {}", employee2.name, res2);

    match can_access_building(&employee3) {
        Ok(access) => println!("Access for {}: {}", employee3.name, access),
        Err(e) => println!("Error: {}", e),
    }
}
