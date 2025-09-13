

mod input;
mod operations;

fn main() {

    let num1 = input::read_number("Enter first number:");
    let op = input::read_operator("Enter operator (+ - * /):");
    let num2 = input::read_number("Enter second number:");

    match operations::calculate(num1, num2, &op) {
        Some(result) => println!("Result: {}", result),
        None => println!("Invalid operator or division by zero!"),
    }





} 

