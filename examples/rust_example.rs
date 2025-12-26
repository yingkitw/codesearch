// Rust example demonstrating various patterns for code search

use std::collections::HashMap;

/// A simple calculator struct
pub struct Calculator {
    history: Vec<String>,
    cache: HashMap<String, f64>,
}

impl Calculator {
    /// Create a new calculator instance
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            cache: HashMap::new(),
        }
    }

    /// Add two numbers
    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = a + b;
        self.history.push(format!("{} + {} = {}", a, b, result));
        result
    }

    /// Multiply two numbers
    pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
        let result = a * b;
        self.history.push(format!("{} * {} = {}", a, b, result));
        result
    }

    /// Calculate factorial recursively
    pub fn factorial(&mut self, n: u32) -> u64 {
        if n <= 1 {
            1
        } else {
            n as u64 * self.factorial(n - 1)
        }
    }

    /// Get calculation history
    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Error types for calculator operations
#[derive(Debug)]
pub enum CalculatorError {
    DivisionByZero,
    InvalidInput(String),
}

/// Main function demonstrating calculator usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut calc = Calculator::new();
    
    // Perform some calculations
    let sum = calc.add(5.0, 3.0);
    let product = calc.multiply(4.0, 7.0);
    let fact = calc.factorial(5);
    
    println!("Sum: {}", sum);
    println!("Product: {}", product);
    println!("Factorial: {}", fact);
    
    // Print history
    for entry in calc.get_history() {
        println!("History: {}", entry);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_calculator_multiply() {
        let mut calc = Calculator::new();
        assert_eq!(calc.multiply(4.0, 5.0), 20.0);
    }

    #[test]
    fn test_factorial() {
        let mut calc = Calculator::new();
        assert_eq!(calc.factorial(5), 120);
    }
}
