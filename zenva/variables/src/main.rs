fn main() {
    let mut value = 9;
    value = 10;
    println!("The value is {}", value);

    let x = 64;
    let x = x+1; // Shadowing
    println!("The value of x is {}", x);

    let x = "RUST PROGRAMMING";
    println!("The value of x is {}", x);

    let small_value:i8 = 100;
    let sample_float:f32 = -700.25;

    println!("Small int {}", small_value);
    println!("Sample float {}", sample_float);

    // Arrays
    let numbers = [1, 2, 3, 4, 5];
    println!("Element at index 0: {}", numbers[0]);

    // Tuples
    let person = ("Alice", 30, 5.4);
    println!("Name:   {}", person.0);
    println!("Age:    {} ", person.1);
    println!("Height: {}", person.2);

    // Constants
    const PI:f32 = 3.14;
    println!("Value of pi {}", PI);
}
