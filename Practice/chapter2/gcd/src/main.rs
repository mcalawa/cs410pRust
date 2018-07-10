/// #Chapter 2: GCD
/// This program is based on code from chapter 2 of Jim Blandy and 
/// Jason Orendorff's *Programming Rust: Fast, Safe Systems 
/// Development*. For a more complicated version of this program that
/// implements more functions for calculations, see calc under the 
/// Homework1 folder
///
/// This program was created with cargo new --bin gcd
/// The --bin marks this as being an application rather than a library
/// This is the command line version of the program

// Trait declarations! Allows us to use certain methods on a type 
// with this trait
use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    // the bang after assert marks this as a macro invocation 
    // rather than a function call
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // let is used for variable declaration 
            // type doesn't have to be specified if it can be inferred
            // in this case it is inferred because m is a u64
            // to explicitly type it you would write let t : u64 = m
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // In Rust, semicolons function as a separator, so one is 
    // generally not used at the end of a return statement.
    // Additionally, it is usually considered bad form to use
    // the word return in a Rust function and the returned value
    // is typically just the last line of the function
    n
}

fn main() {
    // since arrays of different sizes in Rust are different types,
    // they can't be used as a growable type; use Vec, which is Rust's
    // vector type; you'd think the mut would go without saying, but
    // it's actually required because Rust
    let mut numbers = Vec::new();

    // we are skipping one because the first value is always the 
    // name of the program (even if that isn't being typed in)
    // as I learned when making changes to this program for the 
    // first homework, this will freak out on you if you try to 
    // use skip without using from_str, so if you want to grab 
    // strings (or &str or any of Rust's other confusing string
    // variants), check out how I did so in the modified version 
    // for Homework1
    for arg in std::env::args().skip(1) {
        // Rust doesn't have exceptions and errors are handled with
        // Result or panic (more in chapter 7!), so expect is doing
        // error handling (by checking if there was a success) here
        // A successful result returns Ok(v) with v being the value
        // produced and a failure returns Err(e), with e being an 
        // explanation of the error, and exits the program; the 
        // parameter for expect is the error message e
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    // more error checking; writeln writes to the standard error 
    // output stream rather than to the console/command line/whatever
    // and .unwrap() makes sure that the error message writing didn't
    // fail; as in other programming language, 1 represents exiting
    // with an error rather than success (which is 0, as usual)
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    // as is usual, & is for referencing and * is for dereferencing
    // this is especially important in Rust because of the way it 
    // handles memory and ownership; referencing means that ownership
    // stays with numbers and dereferencing means that the value is 
    // being passed to gcd
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

// Rust has unit testing! They can be run with cargo test
// #[test] is an example of an attribute
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}