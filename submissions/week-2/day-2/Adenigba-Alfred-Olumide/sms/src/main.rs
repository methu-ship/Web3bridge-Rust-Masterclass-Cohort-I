pub mod lib;
use crate::lib::{School, Status};


fn main() {
    let mut school = School::new();

    school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);

    let students = school.get_students();

    println!("Students are {:#?}", students);
}