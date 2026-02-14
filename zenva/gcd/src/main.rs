use num::integer;
fn main() {
    let num1: u32 = 150;
    let num2: u32 = 100;

    let gcd = integer::gcd(num1, num2);
    let lcm = integer::lcm(num1, num2);

    println!("The GCD of {} and {} is {}.", num1, num2, gcd);
    println!("The LCM of {} and {} is {}.", num1, num2, lcm);
}
