mod grades;

use grades::Grades;

fn main() {
    
    // Create the grades HashMap.
    let mut grades = Grades::new();

    // Insert some grades.
    grades.add("Charles");
    grades.add("Sally");
    grades.add("Lucy");
    grades.add_with_grade("Linus", 93);

    grades.update("Charles", 72);
    grades.update("Sally", 84);
    grades.update("Lucy", 88);
    grades.update("Linus", 98);
    
    // Find a specific student's grade.
    let student = "Linus";
	grades.print_grade(student);
    grades.print_grade("Snoopy");

    match grades.get("Snoopy") {
        Some(grade) => println!("Snoopy's grade is {}", grade),
        None => println!("Snoopy is not a student"),
    }
    
    // Update a student's grade.
    grades.update("Charles", 78);

    // Remove a student
    grades.remove("Sally");

    // Print all students and their grade
    grades.print_all_grades();
}