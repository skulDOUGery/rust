fn main() {
    /*
        SCENARIO 2: 

        1. Create an array with the temperatures for each day of the week
        2. Use a for loop to calculate sum of temperatures
        3. Find and print the average
     */
    let temperatures = [70.1, 80.2, 75.3, 68.0, 71.8, 77.7, 65.5];
    let mut total = 0.0;
    
    for temperature in temperatures {
        total += temperature;
    }
    let average = total / temperatures.len() as f32;

    println!("Average Temperature: {}", average);
}
