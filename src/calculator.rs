use std::io;

fn main() {
    let mut op = String::new();
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("First number: ");

    io::stdin()
        .read_line(&mut num1)
        .expect("Couldn't read input stream.");


    println!("Second number: ");

    io::stdin()
        .read_line(&mut num2)
        .expect("Couldn't read input stream.");


    println!("Operation: ");

    io::stdin()
    	.read_line(&mut op)
    	.expect("Couldn't read input stream.");

    let a = num1.trim().parse::<i32>().expect("You must input a valid integer.");
    let b = num2.trim().parse::<i32>().expect("You must input a valid integer.");


    if op.trim() == "+" {
        println!("Result: {:?}", a + b);
    }

    if op.trim() == "-" {
        println!("Result: {:?}", a - b);
    }

    if op.trim() == "/" {
        println!("Result: {:?}", a / b);
    }

    if op.trim() == "*" {
        println!("Result: {:?}", a * b);
    }
}