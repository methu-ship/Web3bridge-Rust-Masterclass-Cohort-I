
#[derive(Clone)]



pub struct Student {
  
    pub id: u32,
   
    pub name: String,
  
    pub grade: u8,
   
    pub is_active: bool,
}


pub struct ClassManagement {

    pub students: Vec<Student>
}


impl ClassManagement {
 
    pub fn new() -> ClassManagement {
       
        ClassManagement { 
            students: Vec::new()  
         }
    }

    pub fn register_student(&mut self, student: Student) {
        
        self.students.push(student);
    }
    
 
    pub fn create_student(&mut self, name: String, grade: u8) {
       
        let student = Student {
            id: (self.students.len() + 1) as u32, 
            name,        
            grade,       
            is_active: true,  
        };
       
        self.students.push(student);
    }

    
    pub fn view_all_students(&self) -> Vec<Student> {
   
        self.students.to_vec()
    }


    // pub fn view_student(&self, index: usize) -> &Student {
    
    //     self.students.get(index).unwrap()
    // }


    pub fn fetch_student(&self, index: usize) -> &Student {
       
        if self.students.len() > index {
          
            &self.students[index]
        } else {
          
            panic!("Index out of bounds");
        }
    }
    
    pub fn edit_student(&mut self, index: usize, name: Option<String>, grade: Option<u8>, is_active: Option<bool>) {
        if index < self.students.len() {
            let student = &mut self.students[index];
            
            if let Some(new_name) = name {
                student.name = new_name;
            }
            
            if let Some(new_grade) = grade {
                student.grade = new_grade;
            }
            
            if let Some(new_status) = is_active {
                student.is_active = new_status;
            }
        }
    }

     pub fn update_student_status(&mut self, index: usize, is_active: bool) {
        if index < self.students.len() {
            self.students[index].is_active = is_active;
        }
    }
 

  pub fn delete_student(&mut self, index: usize) {
    
        self.students.remove(index);
    }

     pub fn get_active_students(&self) -> Vec<Student> {
        self.students.iter()
            .filter(|student| student.is_active)
            .cloned()
            .collect()
    }
}



#[cfg(test)]
mod tests {
  
    use super::*;


    #[test]
    fn test_register_student() {
    
        let mut class = ClassManagement::new();

        
        assert!(class.students.len() == 0);

       
        let student = Student { 
            id: 1,
            name: "John Doe".to_string(),
            grade: 10,
            is_active: true,
        };
        
       
        class.register_student(student);
        
      
        assert!(class.students.len() == 1);
    }


 
    #[test]
    fn test_create_student() {
      
        let mut class = ClassManagement::new();
        
       
        class.create_student("Jane Smith".to_string(), 11);
        
       
        assert!(class.students.len() == 1);
    }

 
    #[test]
    fn test_view_all_students() {
    
        let mut class = ClassManagement::new();
        
       
        class.create_student("John Doe".to_string(), 10);
        
       
        let all_students = class.view_all_students();
        
       
        assert!(all_students.len() == 1);
    }

   
    #[test]
    fn test_view_student() {
    
        let mut class = ClassManagement::new();
        
       
        class.create_student("John Doe".to_string(), 10);
        
       
        let student = class.fetch_student(0);
        assert_eq!(student.name, "John Doe");
        assert_eq!(student.grade, 10);
        assert_eq!(student.is_active, true);
    }

}