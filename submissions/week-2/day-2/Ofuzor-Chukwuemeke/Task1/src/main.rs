
enum Status{
    Active,
    InActive
}

struct Class{
    name:String,
    grade:String,
    active:Status
}

struct ClassList{
    students:Vec<Class>
}

impl ClassList{

    fn new()->ClassList{
        ClassList{
            students:Vec::new()
        }
    }

    fn add_student(self,name , grade,active)->ClassList{
        let student = Class{
            name:name.to_string(),
            grade:grade.to_string(),
            active:active
        };
        self.students.push(student);
        self
    }

    fn edit_student(&mut self, index: usize, name: Option<&str>, grade: Option<&str>, active: Option<Status>) {
        if let Some(student) = self.students.get_mut(index) {
            if let Some(name) = name {
                student.name = name.to_string();
            }
            if let Some(grade) = grade {
                student.grade = grade.to_string();
            }
            if let Some(active) = active {
                student.active = active;
            }
        }
    }

    fn remove_student(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        }
    }
}

fn main() {
    let mut class_list = ClassList::new();
    class_list = class_list.add_student("Alice", "A", Status::Active);
    class_list = class_list.add_student("Bob", "B", Status::InActive);
    class_list.edit_student(0, Some("Alice Smith"), None, None);
    class_list.remove_student(1);
    println!("Hello, world!");
}

// Test each of the methods
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_student() {
        let mut class_list = ClassList::new();
        class_list = class_list.add_student("Alice", "A", Status::Active);
        assert_eq!(class_list.students.len(), 1);
    }

    #[test]
    fn test_edit_student() {
        let mut class_list = ClassList::new();
        class_list = class_list.add_student("Alice", "A", Status::Active);
        class_list.edit_student(0, Some("Alice Smith"), None, None);
        assert_eq!(class_list.students[0].name, "Alice Smith");
    }

    #[test]
    fn test_remove_student() {
        let mut class_list = ClassList::new();
        class_list = class_list.add_student("Alice", "A", Status::Active);
        class_list.remove_student(0);
        assert_eq!(class_list.students.len(), 0);
    }
    #[test]
    fn test_remove_student_out_of_bounds() {
        let mut class_list = ClassList::new();
        class_list = class_list.add_student("Alice", "A", Status::Active);
        class_list.remove_student(1); // Attempt to remove a student at an out-of-bounds index
        assert_eq!(class_list.students.len(), 1); // Should still have one student
    }
}