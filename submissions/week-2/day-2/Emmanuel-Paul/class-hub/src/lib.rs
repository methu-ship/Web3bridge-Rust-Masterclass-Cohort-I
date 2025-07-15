use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum StudentStatus {
    Active,
    Inactive,
}

impl fmt::Display for StudentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StudentStatus::Active => write!(f, "Active"),
            StudentStatus::Inactive => write!(f, "Inactive"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grade: String,
    pub status: StudentStatus,
}

impl Student {
    pub fn new(id: u32, name: String, grade: String) -> Self {
        Student {
            id,
            name,
            grade,
            status: StudentStatus::Active,
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}, Name: {}, Grade: {}, Status: {}",
            self.id, self.name, self.grade, self.status
        )
    }
}

#[derive(Debug)]
pub enum ClassManagementError {
    StudentNotFound,
    StudentAlreadyExists,
    InvalidGrade,
    InvalidName,
}

impl fmt::Display for ClassManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassManagementError::StudentNotFound => write!(f, "Student not found"),
            ClassManagementError::StudentAlreadyExists => write!(f, "Student already exists"),
            ClassManagementError::InvalidGrade => write!(f, "Invalid grade provided"),
            ClassManagementError::InvalidName => write!(f, "Invalid name provided"),
        }
    }
}

pub struct ClassManagementSystem {
    students: HashMap<u32, Student>,
    next_id: u32,
}

impl ClassManagementSystem {
    pub fn new() -> Self {
        ClassManagementSystem {
            students: HashMap::new(),
            next_id: 1,
        }
    }

    // Function to register a new student
    pub fn register_student(
        &mut self,
        name: String,
        grade: String,
    ) -> Result<u32, ClassManagementError> {
        // Validate input
        if name.trim().is_empty() {
            return Err(ClassManagementError::InvalidName);
        }

        if grade.trim().is_empty() {
            return Err(ClassManagementError::InvalidGrade);
        }

        let student_id = self.next_id;
        let student = Student::new(
            student_id,
            name.trim().to_string(),
            grade.trim().to_string(),
        );

        self.students.insert(student_id, student);
        self.next_id += 1;

        Ok(student_id)
    }

    // Function to edit student information
    pub fn edit_student(
        &mut self,
        id: u32,
        name: Option<String>,
        grade: Option<String>,
        status: Option<StudentStatus>,
    ) -> Result<(), ClassManagementError> {
        let student = self
            .students
            .get_mut(&id)
            .ok_or(ClassManagementError::StudentNotFound)?;

        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(ClassManagementError::InvalidName);
            }
            student.name = new_name.trim().to_string();
        }

        if let Some(new_grade) = grade {
            if new_grade.trim().is_empty() {
                return Err(ClassManagementError::InvalidGrade);
            }
            student.grade = new_grade.trim().to_string();
        }

        if let Some(new_status) = status {
            student.status = new_status;
        }

        Ok(())
    }

    // Function to update student status
    pub fn update_student_status(
        &mut self,
        id: u32,
        status: StudentStatus,
    ) -> Result<(), ClassManagementError> {
        let student = self
            .students
            .get_mut(&id)
            .ok_or(ClassManagementError::StudentNotFound)?;
        student.status = status;
        Ok(())
    }

    // Function to delete a student
    pub fn delete_student(&mut self, id: u32) -> Result<Student, ClassManagementError> {
        self.students
            .remove(&id)
            .ok_or(ClassManagementError::StudentNotFound)
    }

    // Function to view a specific student
    pub fn view_student(&self, id: u32) -> Result<&Student, ClassManagementError> {
        self.students
            .get(&id)
            .ok_or(ClassManagementError::StudentNotFound)
    }

    // Function to view all students
    pub fn view_all_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }

    // Function to view students by status
    pub fn view_students_by_status(&self, status: StudentStatus) -> Vec<&Student> {
        self.students
            .values()
            .filter(|student| student.status == status)
            .collect()
    }

    // Function to get the total number of students
    pub fn student_count(&self) -> usize {
        self.students.len()
    }

    // Function to check if a student exists
    pub fn student_exists(&self, id: u32) -> bool {
        self.students.contains_key(&id)
    }
}

impl Default for ClassManagementSystem {
    fn default() -> Self {
        Self::new()
    }
}