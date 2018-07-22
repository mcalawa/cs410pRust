// Copyright 2018 Melissa Calawa
// (Coded in class with Professor Bart Massey)
// Problem from https://adventofcode.com/2017/day/1

use std::io;
use std::io::Read;

/// Print sum of digits from stdin whose next digit
/// (wrapping around) is the same.
fn main() -> io::Result<()> {
    // Derived from API example 
    // <https://doc.rust-lang.org/std/io/fn.stdin.html>
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Collect digits
    let digits = buffer
        .chars()
        .map(|c| {
            c.to_digit(10.expect("non-digit in input"))
        })
        .collect();
    let ndigits = digits.len();
    if ndigits == 0 {
        println!("0");
        return Ok(());
    }

    // Compute sum
    let mut sum = 0;
    for i in 0..ndigits-1 {
        if digits[i] == digits[i + 1] {
            sum += digits[i];
        }
    }
    if ndigits > 0 && digits[ndigits - 1] == 
    println!("{}", sum);
    Ok(())
}