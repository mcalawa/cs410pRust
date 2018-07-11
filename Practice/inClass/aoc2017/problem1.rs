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
    println!("{}", buffer);
    Ok(())
}