fn main() {
    /*
        SCENARIO 1: Fibonacci Series (Print first 10 numbers)
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
     */
    let mut count = 0;
    let mut prev = [0, 1];
    
    for val in prev {
        print!("{}, ", val);
        count += 1;
    }

    // Generate the sequence
    while count < 10 {
        let next = prev[0] + prev[1];
        print!("{}, ", next);
        prev[0] = prev[1];
        prev[1] = next;
        count += 1;
    } 

    println!("...");
}
