mod financial_calculations;
fn main() {
    let income = 45000.0;
    let tax = financial_calculations::calculate_income_tax(income);
    println!("Annual Income Tax: ${:.2}", tax);
    let principal = 1000.0;
    let rate = 0.05;
    let time = 10.0;
    let n = 4.0;
    let compound_amount = financial_calculations::calculate_compound_interest(principal, rate, time, n);
    println!("Compound Interest Amount: ${:.2}", compound_amount);
    let simple_amount = financial_calculations::calculate_simple_interest(principal, rate, time);
    println!("Simple Interest Amount: ${:.2}", simple_amount);
    let tax_rate = financial_calculations::get_tax_rate(income);
    println!("Tax Rate: {:.2}%", tax_rate * 100.0);
}
