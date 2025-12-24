use std::collections::HashMap;

pub struct Grades {
    map: HashMap<String, i32>,
}

#[allow(dead_code)]
impl Grades {
    // Constructor
    pub fn new() -> Self {
        Self { 
            map: HashMap::new() 
        }
    }

    // Add a student.
    pub fn add(&mut self, student: &str) {
        self.map.entry(student.to_string()).or_insert(0);
    }

    pub fn add_with_grade(&mut self, student: &str, grade: i32) {
        self.map.entry(student.to_string()).or_insert(grade);
    }

    // Update a student's grade.
    pub fn update(&mut self, student: &str, new_grade: i32) {
        self.map.insert(student.to_string(), new_grade);
    }

    // Remove a student
    pub fn remove(&mut self, student: &str) {
        self.map.remove(student);
    }

    // Get a student's grade
    pub fn get(&self, student: &str) -> Option<i32> {
        self.map.get(student).copied()
    }

    // Print a single student's grade.
    pub fn print_grade(&self, student: &str) {
        match self.map.get(student) {
            Some(grade) => println!("The grade for {} is: {}.", student, grade),
            None => println!("{} not found.", student),
        }
    }

    // Print all students and their grades
    pub fn print_all_grades(&self) {
        for (student, grade) in &self.map {
            println!("{}: {}", student, grade);
        }
    }
}
