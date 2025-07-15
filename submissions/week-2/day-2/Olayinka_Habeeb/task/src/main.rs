f
struct Student {
    std_name: String,
    std_grade: u16,
    status: StudentStatus,
}

#[derive(Debug)]
enum StudentStatus {
    Active,
    Inactive,
}

impl Student {
    fn register_student(name: String, grade: u16) -> Student {
        Student {
            std_name: name,
            std_grade: grade,
            status: StudentStatus::Active,
        }
    }

    fn update_student(&mut self, new_status: StudentStatus) {
        self.status = new_status;
    }

    fn edit(&mut self, new_name: String, new_grade: u16) {
        self.std_name = new_name;
        self.std_grade = new_grade;
    }

    fn delete_student(&mut self) {
        self.status = StudentStatus::Inactive;
    }

    fn view_student(&self) {
        println!("StudentName ==> {}", self.std_name);
        println!("StudentGrade ==> {}", self.std_grade);
        println!("Status ==> {:?}", self.status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let student = Student::register_student("Yinka".to_string(), 10);
        assert_eq!(student.std_name, "Yinka");
        assert_eq!(student.std_grade, 10);
        assert!(matches!(student.status, StudentStatus::Active));
    }
}
