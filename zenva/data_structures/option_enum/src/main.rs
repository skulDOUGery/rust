fn safe_divide(numerator: i32, denominator: i32) -> Option<i32> {
    if denominator == 0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = safe_divide(10, 2);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot Divide by Zero"),
    }

    let no_result = safe_divide(10, 0);
    match no_result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot Divide by Zero"),
    }
}
