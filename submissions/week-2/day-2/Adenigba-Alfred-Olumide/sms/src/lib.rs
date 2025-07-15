// school management system

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub age: u32,
    pub class: String,
    pub student_status: Status,
}

pub struct School {
    pub students: Vec<Student>,
}

impl School {
    pub fn new() -> School {
        School {
            students: Vec::new(),
        }
    }

    pub fn create_student(&mut self, name: String, age: u32, class: String, student_status: Status) {
        let student = Student {
            name,
            age,
            class,
            student_status,
        };
        self.students.push(student);
    }

    pub fn edit_student(&mut self, index: usize, name: Option<String>, age: Option<u32>, class: Option<String>, student_status: Option<Status>) {
        if let Some(student) = self.students.get_mut(index) {
            if let Some(n) = name {
                student.name = n;
            }
            if let Some(a) = age {
                student.age = a;
            }
            if let Some(c) = class {
                student.class = c;
            }
            if let Some(s) = student_status {
                student.student_status = s;
            }
        } else {
            panic!("Student index out of bounds");
        }
    }

    pub fn delete_student(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        } else {
            panic!("Student index out of bounds");
        }
    }

    pub fn get_students(&self) -> Vec<Student> {
        self.students.to_vec()
    }

    pub fn get_student(&self, index: usize) -> &Student {
        self.students.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_student() {
        let mut school = School::new();

        assert!(school.students.len() == 0);
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);

        assert!(school.students.len() == 1);
    }

    #[test]
    fn test_get_students() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);
        school.create_student("Olaide Adenigba".to_string(), 12, "SS1".to_string(), Status::Active);

        let students = school.get_students();
        assert!(students.len() == 2);
    }

    #[test]
    fn test_delete_student() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);
        school.create_student("Olaide Adenigba".to_string(), 12, "SS1".to_string(), Status::Active);

        school.delete_student(0);
        let students = school.get_students();
        assert!(students.len() == 1);
    }

    #[test]
    fn get_student() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);

        let student = school.get_student(0);
        assert!(student.name == "Olumide Adenigba");
        assert!(student.age == 14);
        assert!(student.class == "SS3");
        assert!(student.student_status == Status::Active);
    }

    #[test]
    fn test_edit_student() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);

        school.edit_student(0, Some("Olaide Adenigba".to_string()), Some(12), Some("SS1".to_string()), Some(Status::Active));

        let student = school.get_student(0);
        assert!(student.name == "Olaide Adenigba");
        assert!(student.age == 12);
        assert!(student.class == "SS1");
        assert!(student.student_status == Status::Active);
    }
    
}