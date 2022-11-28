use core::panic;
//simple calculator that requests 2 numbers and an operation from the user
use std::{io::{stdin,stdout,Write}, result};

//try using a vec to store operation types

fn read(input: &mut String){
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    println!("This is a calculator");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    let operators: Vec<char> = vec!['+','-','*','/'];

    print!("Please provide the first number ");
    read(&mut num1);
    print!("Please provide the second number ");
    read(&mut num2);
    print!("Please provide the operation you would like to perform ");
    read(&mut operator);

    let num1: f32 = num1.trim().parse().unwrap();
    let num2: f32 = num2.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    assert!((operators.contains(&operator)), "You have chosen an invalid operator");

    let result = match operator{
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num2 / num2,
        _ => panic!("cannot find operator, there is an error"),
    };

    println!("The result of {} {} {} is {}", num1, operator, num2, result);


}
