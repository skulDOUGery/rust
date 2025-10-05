fn main() {
    /* 1. Create an array of student scores */
    let scores = [85, 90, 78, 92, 88];

    /* 2. Use the for loop to iterate through the array and call get_letter_grade_function */
    for score in scores {
        let grade = get_letter_grade(score);
        println!("Sccore: {} - {}", score, grade);
    }
}

fn get_letter_grade(score:u8) -> char {
    if score >= 90 && score <= 100 {
        'A'
    }
    else if score >= 80 && score <= 89 {
        'B'
    }
    else if score >= 70 && score <= 79 {
        'C'
    }
    else if score >= 60 && score <= 69 {
        'D'
    }
    else {
        'F'
    }
}