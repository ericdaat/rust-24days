extern crate primal;

use self::primal::Sieve;
use std::io;

pub fn check_if_prime() {
    let sieve = Sieve::new(10000);
    let mut input = String::new();

    println!("please type a number to check if it's a prime");
    io::stdin().read_line(&mut input)
        .expect("failed to read number");

    let suspect: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("can't parse this"),
    };

    if sieve.is_prime(suspect as usize) {
        println!("it's a prime!");
    } else {
        println!("it's not a prime");
    };
}

