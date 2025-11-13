use std::io::{self, Read};

fn main() {
    println!("Enter first number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read");
    let num1: f64 = input1.trim().parse().expect("Please enter a number");
    
    println!("Please enter aan operator (+, -, *, /)");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read");
    let op = op.trim();

    println!("Enter second number");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read");
    let num2:f64 = input2.trim().parse().expect("Please enter a number");

    let result = match op{
        "+" => num1+num2,
        "-" => num1-num2,
        "*" => num1*num2,
        "/" => num1/num2,
        _ =>{
            println!("Invalid operator");
            return;
        }
    };
    println!("Result:{}",result);

}
