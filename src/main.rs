use std::io::{stdin};

// A simple fibonacci calculator app

fn fib_series (fib_number: u128) -> Vec<u128> {
    let mut fibs: Vec<u128> = Vec::new();
    let mut start_fibs: u128 = 0;
    while start_fibs < fib_number {
        match start_fibs {
            0 => {
                fibs.push(0);
            },
            1 | 2 => {
                fibs.push(1);
            },
            _ => {
                fibs.push((&fibs[start_fibs as usize - 1]) + (&fibs[start_fibs as usize - 2]));
            }
        }
        start_fibs += 1;
    }
    fibs
}

fn main() {
    let mut fib_number = String::new();
    println!("Enter the Fibonacci whole number to calculate it's Fibonacci series");
    stdin().read_line(&mut fib_number).expect("Failed to read Fibonacci value");
    let fib_number: u128 = match fib_number.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Please enter a positive whole number");
            return;
        },
    };

    println!("The Fibonacci series upto {} is: {:?}", fib_number, fib_series(fib_number));
}
