fn main() {
    /*
        IF STATEMENT:
        If a number is divisible by 5 and 3 print 'TriQuint'
        If a number is divisible by 6 and 4 print 'HexaQuad'
        Otherwise print 'Just another number'
     */
    let number = 36;
    if number % 5 == 0 && number % 3 == 0 {
        println!("{} is a TriQuint", number);
    } else if number % 6 == 0 && number % 4 == 0 {
        println!("{} is a HexaQuad", number);
    } else {
        println!("{} is just another number", number);
    }

    /*
        IF LET STATEMENT:
        If is_weekend is true, assign "Go Hiking" to the variable 'activity'
        otherwise, assign 'Go to work'  
     */
    let is_weekend:bool = false;
    let activity = if is_weekend {"Go Hiking"} else {"Go to work"};
    println!("{} is today's activity", activity);
}
