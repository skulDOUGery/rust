fn read_file(file_path: &str) -> Result<String, String> {
    if file_path == "valid.txt" {
        Ok(String::from("<File content here>"))
    } else {
        Err(String::from("File not found!"))
    }
}

fn main() {
    let file_result = read_file("valid.txt");
    match file_result {
        Ok(content) => println!("File read done: {}", content),
        Err(error) => println!("Error: {}", error),
    }

    let invalid_file_result = read_file("invalid.txt");
    match invalid_file_result {
        Ok(content) => println!("File read done: {}", content),
        Err(error) => println!("Error: {}", error),
    }
}
