
pub enum Status {
    Active,
    NotActive
}

pub enum Grade {
    A, B, C, D
}

pub struct Student {
    pub name: String,
    pub grade: Grade,
    pub status: Status
}

pub struct Management {
    pub students: Vec<Student>
}

impl Management {
    pub fn init() -> Self {
        Management {
            students: Vec::new(),
        }
    }

    fn reg_stu (&mut self, stud: Student) {
        self.students.push(stud)
    }

    fn edit_stu (&mut self, edit_stu: String) {
        self.students:name = edit_stu;
    }
}
