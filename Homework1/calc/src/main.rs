use std::*;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn sum(n: u64, m: u64) -> u64 {
    n + m
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(2) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        std::process::exit(0);
    }
    else if numbers.len() == 1 {
        println!("{}", numbers[0]);
    }
    else if arguments[1].to_string() == "gcd" {
        let mut d = numbers[0];
        for m in &numbers[1..] {
            d = gcd(d, *m);
        }

        println!("{:?}", d);
    }
    else if arguments[1] == "sum" {
        let mut s = numbers[0];
        for m in &numbers[1..] {
            s = sum(s, *m);
        }

        println!("{:?}", s);
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[test]
fn test_sum() {
    assert_eq!(sum(15, 20), 35);

    assert_eq!(sum(1 + 2 + 3 + 4 + 5, 6 + 7 + 8 + 9), 10 + 20 + 10 + 5);
}