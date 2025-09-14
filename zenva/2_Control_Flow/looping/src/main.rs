fn main() {
    /*
        FOR LOOP:
        Iterate over a collection, like an array.
     */
    let arr = [10, 20, 30, 40, 50];
    for elem in arr {
        println!("{}", elem);
    }

    /*
        WHILE LOOP:
        Print a count down and when counter reaches zero, print "LIFT OFF"!
     */
    let mut counter = 10;
    while counter > 0 {
        println!("Countdown: {}", counter);
        counter -= 1;
    }
    println!("LIFT OFF!");

    /* LOOP STATEMENT */
    let mut index = 1;
    loop {
        index += 1;
        println!("Index: {}", index);

        if index == 100 {
            println!("Max index reached");
            break;
        }
    }
}
