mod lib;

use crate::lib::{can_access_building, Department, EmployeeDetails, Status};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let employee1 = EmployeeDetails {
        name: "Alice".to_string(),
        department: Department::MEDIA,
        is_terminated: Status::ACTIVE,
    };

    let employee2 = EmployeeDetails {
        name: "Bob".to_string(),
        department: Department::SOCIAL_MEDIA,
        is_terminated: Status::TERMINATED,
    };

    let employee3 = EmployeeDetails {
        name: "Rick".to_string(),
        department: Department::MANAGER,
        is_terminated: Status::TERMINATED,
    };

    let res = can_access_building(&employee1).unwrap();
    println!("Access for {}: {}", employee1.name, res);


    let res2 = can_access_building(&employee2)?;
    println!("Access for {}: {}", employee2.name, res2);

    Ok(())
}
