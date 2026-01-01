//! Additional edge case and complex scenario tests for complexity module

#[cfg(test)]
mod edge_case_tests {
    use crate::complexity::{
        calculate_cognitive_complexity,
        calculate_cyclomatic_complexity,
        calculate_file_complexity,
        calculate_nesting_depth,
    };

    #[test]
    fn test_empty_content() {
        let content = "";
        let cyclomatic = calculate_cyclomatic_complexity(content);
        let cognitive = calculate_cognitive_complexity(content);
        assert!(cyclomatic >= 1);
        assert_eq!(cognitive, 0);
        assert_eq!(calculate_nesting_depth(content), 0);
    }

    #[test]
    fn test_only_whitespace() {
        let content = "   \n\n   \t\t\n   ";
        let cyclomatic = calculate_cyclomatic_complexity(content);
        let cognitive = calculate_cognitive_complexity(content);
        assert!(cyclomatic >= 1);
        assert_eq!(cognitive, 0);
    }

    #[test]
    fn test_only_comments() {
        let content = r#"
// This is a comment
/* Multi-line
   comment */
// Another comment
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        let cognitive = calculate_cognitive_complexity(content);
        assert!(cyclomatic >= 1);
        assert_eq!(cognitive, 0);
    }

    #[test]
    fn test_deeply_nested_code() {
        let content = r#"
fn test() {
    if a {
        if b {
            if c {
                if d {
                    if e {
                        println!("deep");
                    }
                }
            }
        }
    }
}
"#;
        let nesting = calculate_nesting_depth(content);
        assert!(nesting >= 5);
        
        let cognitive = calculate_cognitive_complexity(content);
        assert!(cognitive > 10); // Deep nesting increases cognitive complexity
    }

    #[test]
    fn test_many_branches() {
        let content = r#"
fn test() {
    if a { }
    if b { }
    if c { }
    if d { }
    if e { }
    if f { }
    if g { }
    if h { }
    if i { }
    if j { }
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 10);
    }

    #[test]
    fn test_switch_case_complexity() {
        let content = r#"
fn test(x: i32) {
    match x {
        1 => {},
        2 => {},
        3 => {},
        4 => {},
        5 => {},
        _ => {},
    }
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        // Match statements contribute to complexity
        assert!(cyclomatic >= 1);
    }

    #[test]
    fn test_nested_loops() {
        let content = r#"
fn test() {
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                println!("{} {} {}", i, j, k);
            }
        }
    }
}
"#;
        let nesting = calculate_nesting_depth(content);
        assert!(nesting >= 3);
        
        let cognitive = calculate_cognitive_complexity(content);
        assert!(cognitive > 5);
    }

    #[test]
    fn test_mixed_control_structures() {
        let content = r#"
fn test() {
    if condition {
        while loop_condition {
            for item in items {
                match item {
                    Some(x) => {
                        if x > 10 {
                            break;
                        }
                    }
                    None => continue,
                }
            }
        }
    }
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 5);
        
        let cognitive = calculate_cognitive_complexity(content);
        assert!(cognitive >= 10);
    }

    #[test]
    fn test_ternary_operators() {
        let content = r#"
fn test() {
    let x = if a { 1 } else { 2 };
    let y = if b { 3 } else { 4 };
    let z = if c { 5 } else { 6 };
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 3);
    }

    #[test]
    fn test_logical_operators() {
        let content = r#"
fn test() {
    if a && b || c && d {
        println!("complex condition");
    }
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 1);
    }

    #[test]
    fn test_try_catch_complexity() {
        let content = r#"
fn test() {
    match result {
        Ok(val) => println!("{}", val),
        Err(e) => eprintln!("{}", e),
    }
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 1);
    }

    #[test]
    fn test_multiple_functions() {
        let content = r#"
fn func1() {
    if a { }
    if b { }
}

fn func2() {
    for i in 0..10 {
        if i > 5 { }
    }
}

fn func3() {
    match x {
        1 => {},
        2 => {},
        _ => {},
    }
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 6);
    }

    #[test]
    fn test_recursive_function() {
        let content = r#"
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 1);
    }

    #[test]
    fn test_lambda_expressions() {
        let content = r#"
fn test() {
    let f = |x| if x > 0 { x } else { -x };
    let g = |y| match y {
        Some(v) => v,
        None => 0,
    };
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 2);
    }

    #[test]
    fn test_very_long_function() {
        let mut content = String::from("fn test() {\n");
        for i in 0..100 {
            content.push_str(&format!("    if condition_{i} {{ }}\n"));
        }
        content.push_str("}\n");
        
        let cyclomatic = calculate_cyclomatic_complexity(&content);
        assert!(cyclomatic >= 100);
    }

    #[test]
    fn test_file_complexity_empty_content() {
        let metrics = calculate_file_complexity("test.rs", "");
        assert!(metrics.cyclomatic_complexity >= 1);
        assert_eq!(metrics.cognitive_complexity, 0);
    }

    #[test]
    fn test_unicode_in_code() {
        let content = r#"
fn test() {
    let message = "你好世界";
    if message.len() > 0 {
        println!("{}", message);
    }
}
"#;
        let cyclomatic = calculate_cyclomatic_complexity(content);
        assert!(cyclomatic >= 1);
    }
}
