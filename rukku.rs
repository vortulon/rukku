// first attempt at making something in rust
// a simple calculator that takes two numbers and an operation
// and returns the result
// i like rust
// but ill stay a bit more with lua for now
use std::io;

fn main() {
    println!("rukku calculator");
    println!("enter two numbers and an operation (+, -, *, /)");

    loop {
        // get first number
        println!("\nenter first number:");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("failed to read line");
        
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };

        // get second number
        println!("enter second number:");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("failed to read line");
        
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };

        // get operation
        println!("enter operation (+, -, *, /):");
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("failed to read line");

        // calculate and display result
        let result = match operation.trim() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("cannot divide by zero!");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("invalid operation! use +, -, *, or /");
                continue;
            }
        };

        println!("result: {}", result);

        // ask if user wants to continue
        println!("calculate again? (yes/no)");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("failed to read line");

        if answer.trim().to_lowercase() != "yes" {
            println!("goodbye!");
            break;
        }
    }
}
