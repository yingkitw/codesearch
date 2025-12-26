// Circular Call Demo
// This file demonstrates circular/recursive function calls for testing the circular detection

// Simple direct recursion
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Mutual recursion: is_even and is_odd call each other
fn is_even(n: u32) -> bool {
    if n == 0 {
        true
    } else {
        is_odd(n - 1)
    }
}

fn is_odd(n: u32) -> bool {
    if n == 0 {
        false
    } else {
        is_even(n - 1)
    }
}

// Three-way circular: process_a -> process_b -> process_c -> process_a
fn process_a(data: &str) -> String {
    let processed = format!("A:{}", data);
    process_b(&processed)
}

fn process_b(data: &str) -> String {
    let processed = format!("B:{}", data);
    process_c(&processed)
}

fn process_c(data: &str) -> String {
    let processed = format!("C:{}", data);
    // This creates a circular dependency
    if processed.len() < 100 {
        process_a(&processed)
    } else {
        processed
    }
}

// Normal function - no circular call
fn helper(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Factorial: {}", factorial(5));
    println!("Is 4 even? {}", is_even(4));
    println!("Is 5 odd? {}", is_odd(5));
    println!("Processed: {}", process_a("start"));
    println!("Helper: {}", helper(10));
}

