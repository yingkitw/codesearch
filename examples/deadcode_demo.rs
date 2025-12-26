// Dead Code Demonstration File
// This file intentionally contains dead code for testing the deadcode detection feature

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================
// DEAD CODE EXAMPLES
// ============================================

// Unused struct
struct UnusedConfig {
    debug_mode: bool,
    max_retries: u32,
    timeout: u64,
}

// Unused enum
enum Status {
    Pending,
    Running,
    Complete,
    Failed,
}

// Unused function
fn unused_helper(x: i32, y: i32) -> i32 {
    x * y + 100
}

// Another unused function
fn deprecated_format(s: &str) -> String {
    format!("[DEPRECATED] {}", s)
}

// Unused constant
const MAX_BUFFER_SIZE: usize = 4096;
const DEFAULT_TIMEOUT: u64 = 30;

// ============================================
// USED CODE (for comparison)
// ============================================

pub struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn add(&mut self, a: i32, b: i32) -> i32 {
        let result = a + b;
        self.history.push(format!("{} + {} = {}", a, b, result));
        result
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }
}

fn main() {
    let mut calc = Calculator::new();
    let sum = calc.add(5, 3);
    println!("Sum: {}", sum);
    
    for entry in calc.get_history() {
        println!("{}", entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
    }
}

