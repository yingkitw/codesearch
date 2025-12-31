//! Detection functions for various types of dead code

use super::types::DeadCodeItem;
use super::helpers::{is_special_function, truncate_string, is_commented_out_code, extract_import_name};
use crate::parser::get_file_extension;
use crate::language::get_language_by_extension;
use regex::Regex;

/// Detect unused variables in a file
pub fn detect_unused_variables(file_path: &str, content: &str, items: &mut Vec<DeadCodeItem>) {
    let ext = get_file_extension(file_path);
    let lang = get_language_by_extension(ext);
    
    if lang.is_none() {
        return;
    }
    
    let variable_patterns = [
        (r"let\s+(mut\s+)?(\w+)\s*=", 2),
        (r"const\s+(\w+)\s*=", 1),
        (r"var\s+(\w+)\s*=", 1),
        (r"(\w+)\s*:=\s*", 1),
        (r"(\w+)\s*<-\s*", 1),
    ];
    
    for (pattern, group) in &variable_patterns {
        if let Ok(re) = Regex::new(pattern) {
            for (line_num, line) in content.lines().enumerate() {
                if let Some(caps) = re.captures(line) {
                    if let Some(var_name) = caps.get(*group) {
                        let name = var_name.as_str();
                        if name.starts_with('_') || name == "err" || name.len() <= 1 {
                            continue;
                        }
                        
                        let usage_count = content.matches(name).count();
                        if usage_count <= 1 {
                            items.push(DeadCodeItem {
                                file: file_path.to_string(),
                                line_number: line_num + 1,
                                item_type: "variable".to_string(),
                                name: name.to_string(),
                                reason: "Variable declared but never used".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Detect unreachable code after return statements
pub fn detect_unreachable_code(file_path: &str, content: &str, items: &mut Vec<DeadCodeItem>) {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_function = false;
    let mut brace_depth = 0;
    let mut found_return = false;
    
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        if trimmed.contains("fn ") || trimmed.contains("function ") || trimmed.contains("def ") {
            in_function = true;
            brace_depth = 0;
            found_return = false;
        }
        
        if in_function {
            brace_depth += trimmed.matches('{').count() as i32;
            brace_depth -= trimmed.matches('}').count() as i32;
            
            if trimmed.starts_with("return") || trimmed.contains(" return ") {
                if !trimmed.ends_with(';') && !trimmed.ends_with('{') {
                    continue;
                }
                found_return = true;
            }
            
            if found_return && brace_depth > 0 {
                if !trimmed.is_empty() && !trimmed.starts_with('}') && !trimmed.starts_with("//") && !trimmed.starts_with('#') {
                    if line_num + 1 < lines.len() {
                        let next_line = lines[line_num + 1].trim();
                        if !next_line.starts_with('}') {
                            items.push(DeadCodeItem {
                                file: file_path.to_string(),
                                line_number: line_num + 2,
                                item_type: "unreachable".to_string(),
                                name: truncate_string(next_line, 40),
                                reason: "Code after return statement is unreachable".to_string(),
                            });
                            found_return = false;
                        }
                    }
                }
            }
            
            if brace_depth == 0 {
                in_function = false;
                found_return = false;
            }
        }
    }
}

/// Detect empty functions
pub fn detect_empty_functions(file_path: &str, content: &str, items: &mut Vec<DeadCodeItem>) {
    let lines: Vec<&str> = content.lines().collect();
    
    let function_patterns = [
        r"fn\s+(\w+)\s*\(",
        r"function\s+(\w+)\s*\(",
        r"def\s+(\w+)\s*\(",
        r"func\s+(\w+)\s*\(",
        r"proc\s+(\w+)\s*\(",
    ];
    
    for pattern in &function_patterns {
        if let Ok(re) = Regex::new(pattern) {
            for (line_num, line) in lines.iter().enumerate() {
                if let Some(caps) = re.captures(line) {
                    if let Some(func_name) = caps.get(1) {
                        let name = func_name.as_str();
                        
                        if is_special_function(name) {
                            continue;
                        }
                        
                        let mut found_opening = false;
                        let mut brace_count = 0;
                        let mut colon_count = 0;
                        let mut has_content = false;
                        
                        for i in line_num..lines.len().min(line_num + 20) {
                            let check_line = lines[i].trim();
                            
                            if check_line.contains('{') {
                                found_opening = true;
                                brace_count += check_line.matches('{').count() as i32;
                            }
                            brace_count -= check_line.matches('}').count() as i32;
                            
                            if check_line.ends_with(':') && i == line_num {
                                found_opening = true;
                                colon_count = 1;
                            }
                            
                            if found_opening && (brace_count > 0 || colon_count > 0) {
                                let content_line = if i == line_num {
                                    check_line.split('{').last().unwrap_or("").trim()
                                } else {
                                    check_line
                                };
                                
                                if !content_line.is_empty() 
                                    && !content_line.starts_with('}') 
                                    && !content_line.starts_with("//") 
                                    && !content_line.starts_with('#')
                                    && content_line != "pass"
                                    && !content_line.ends_with(':') {
                                    has_content = true;
                                }
                            }
                            
                            if colon_count > 0 && i > line_num {
                                let next_line_indented = check_line.starts_with(' ') || check_line.starts_with('\t');
                                if !check_line.is_empty() && !next_line_indented {
                                    colon_count = 0;
                                }
                            }
                            
                            if found_opening && brace_count == 0 && colon_count == 0 {
                                if !has_content {
                                    items.push(DeadCodeItem {
                                        file: file_path.to_string(),
                                        line_number: line_num + 1,
                                        item_type: "empty".to_string(),
                                        name: name.to_string(),
                                        reason: "Empty function with no implementation".to_string(),
                                    });
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Detect TODO/FIXME markers
pub fn detect_todo_fixme(file_path: &str, content: &str, items: &mut Vec<DeadCodeItem>) {
    let markers = [
        ("TODO", "TODO marker - incomplete implementation"),
        ("FIXME", "FIXME marker - needs fixing"),
        ("HACK", "HACK marker - temporary workaround"),
        ("XXX", "XXX marker - problematic code"),
        ("BUG", "BUG marker - known bug"),
    ];
    
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        for (marker, reason) in &markers {
            if trimmed.contains(marker) && (trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*")) {
                items.push(DeadCodeItem {
                    file: file_path.to_string(),
                    line_number: line_num + 1,
                    item_type: "todo".to_string(),
                    name: truncate_string(trimmed, 50),
                    reason: reason.to_string(),
                });
                break;
            }
        }
    }
}

/// Detect dead code patterns (commented code and unused imports)
pub fn detect_dead_code_patterns(file_path: &str, content: &str, items: &mut Vec<DeadCodeItem>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        // Detect commented-out code
        if is_commented_out_code(trimmed) {
            items.push(DeadCodeItem {
                file: file_path.to_string(),
                line_number: line_num + 1,
                item_type: "commented code".to_string(),
                name: truncate_string(trimmed, 40),
                reason: "Commented-out code should be removed".to_string(),
            });
        }
        
        // Detect unused imports
        if (trimmed.starts_with("use ") || trimmed.starts_with("import ") || trimmed.starts_with("from ")) 
            && !trimmed.contains('*') 
        {
            if let Some(imported) = extract_import_name(trimmed) {
                let usage_count = content.matches(&imported).count();
                if usage_count <= 1 {
                    items.push(DeadCodeItem {
                        file: file_path.to_string(),
                        line_number: line_num + 1,
                        item_type: "import".to_string(),
                        name: imported,
                        reason: "Imported but never used".to_string(),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_unused_variables() {
        let content = r#"
fn test() {
    let unused_var = 10;
    let used_var = 20;
    println!("{}", used_var);
    let _ignored = 30;
}
"#;
        let mut items = Vec::new();
        detect_unused_variables("test.rs", content, &mut items);
        
        assert!(items.iter().any(|i| i.name == "unused_var"));
        assert!(!items.iter().any(|i| i.name == "used_var"));
        assert!(!items.iter().any(|i| i.name == "_ignored"));
    }

    #[test]
    fn test_detect_unreachable_code() {
        let content = r#"
fn test() {
    if true {
        return 42;
        println!("unreachable");
    }
}
"#;
        let mut items = Vec::new();
        detect_unreachable_code("test.rs", content, &mut items);
        
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.item_type == "unreachable"));
    }

    #[test]
    fn test_detect_empty_functions() {
        let content = r#"fn empty_function() {
}

fn non_empty() {
    println!("hello");
}

fn main() {
}
"#;
        let mut items = Vec::new();
        detect_empty_functions("test.rs", content, &mut items);
        
        assert!(items.iter().any(|i| i.name == "empty_function"));
        assert!(!items.iter().any(|i| i.name == "non_empty"));
        assert!(!items.iter().any(|i| i.name == "main"));
    }

    #[test]
    fn test_detect_todo_fixme() {
        let content = r#"
// TODO: implement this feature
fn test() {
    // FIXME: bug here
    let x = 10;
    // HACK: temporary workaround
    // XXX: problematic code
}
"#;
        let mut items = Vec::new();
        detect_todo_fixme("test.rs", content, &mut items);
        
        assert!(items.iter().any(|i| i.name.contains("TODO")));
        assert!(items.iter().any(|i| i.name.contains("FIXME")));
        assert!(items.iter().any(|i| i.name.contains("HACK")));
        assert!(items.iter().any(|i| i.name.contains("XXX")));
    }

    #[test]
    fn test_detect_dead_code_patterns() {
        let content = r#"
// let commented_var = 10;
use unused_import;
fn test() {
    let x = 10;
}
"#;
        let mut items = Vec::new();
        detect_dead_code_patterns("test.rs", content, &mut items);
        
        assert!(items.iter().any(|i| i.item_type == "commented code"));
    }

    #[test]
    fn test_multi_language_support() {
        let python_content = r#"def empty_python_func():
    pass

def non_empty_python():
    print("hello")
"#;
        let mut items = Vec::new();
        detect_empty_functions("test.py", python_content, &mut items);
        
        assert!(items.iter().any(|i| i.name == "empty_python_func"));
        assert!(!items.iter().any(|i| i.name == "non_empty_python"));
    }

    // Edge case tests
    #[test]
    fn test_detect_unused_variables_edge_cases() {
        // Test with underscore prefix (should be ignored)
        let content = r#"
fn test() {
    let _unused = 10;
    let unused_var = 20;
}
"#;
        let mut items = Vec::new();
        detect_unused_variables("test.rs", content, &mut items);
        assert!(!items.iter().any(|i| i.name == "_unused"));
        assert!(items.iter().any(|i| i.name == "unused_var"));
    }

    #[test]
    fn test_detect_unused_variables_single_char() {
        // Single character variables should be ignored
        let content = "let x = 10;";
        let mut items = Vec::new();
        detect_unused_variables("test.rs", content, &mut items);
        assert!(items.is_empty());
    }

    #[test]
    fn test_detect_unused_variables_err_keyword() {
        // 'err' variable should be ignored
        let content = "let err = Error::new();";
        let mut items = Vec::new();
        detect_unused_variables("test.rs", content, &mut items);
        assert!(items.is_empty());
    }

    #[test]
    fn test_detect_unreachable_code_nested_blocks() {
        let content = r#"
fn test() {
    if true {
        if false {
            return;
            println!("unreachable nested");
        }
    }
}
"#;
        let mut items = Vec::new();
        detect_unreachable_code("test.rs", content, &mut items);
        assert!(!items.is_empty());
    }

    #[test]
    fn test_detect_unreachable_code_no_semicolon() {
        // Return without semicolon should not trigger
        let content = r#"
fn test() -> i32 {
    return 42
}
"#;
        let mut items = Vec::new();
        detect_unreachable_code("test.rs", content, &mut items);
        assert!(items.is_empty());
    }

    #[test]
    fn test_detect_empty_functions_with_comments() {
        let content = r#"
fn empty_with_comment() {
    // This is just a comment
}

fn empty_no_comment() {
}
"#;
        let mut items = Vec::new();
        detect_empty_functions("test.rs", content, &mut items);
        // Both should be detected as empty (comments don't count as content)
        assert!(items.iter().any(|i| i.name == "empty_with_comment"));
        assert!(items.iter().any(|i| i.name == "empty_no_comment"));
    }

    #[test]
    fn test_detect_empty_functions_single_line() {
        let content = "fn empty() {}";
        let mut items = Vec::new();
        detect_empty_functions("test.rs", content, &mut items);
        assert!(items.iter().any(|i| i.name == "empty"));
    }

    #[test]
    fn test_detect_todo_fixme_multiple_markers() {
        let content = r#"
// TODO: FIXME: Multiple markers in one line
fn test() {
    // BUG: XXX: HACK: All markers
}
"#;
        let mut items = Vec::new();
        detect_todo_fixme("test.rs", content, &mut items);
        // Should detect at least the first marker on each line
        assert!(items.len() >= 2);
    }

    #[test]
    fn test_detect_todo_fixme_in_strings() {
        // Should NOT detect markers in strings
        let content = r#"
fn test() {
    let msg = "TODO: This is in a string";
}
"#;
        let mut items = Vec::new();
        detect_todo_fixme("test.rs", content, &mut items);
        assert!(items.is_empty());
    }

    // Complex scenario tests
    #[test]
    fn test_complex_nested_functions() {
        let content = r#"
fn outer() {
    fn inner() {
        return;
        println!("unreachable in inner");
    }
    let unused = 10;
}
"#;
        let mut items = Vec::new();
        detect_unreachable_code("test.rs", content, &mut items);
        detect_unused_variables("test.rs", content, &mut items);
        
        assert!(items.iter().any(|i| i.item_type == "unreachable"));
        assert!(items.iter().any(|i| i.name == "unused"));
    }

    #[test]
    fn test_complex_multiple_returns() {
        let content = r#"
fn test() {
    return 1;
    println!("unreachable after return");
}
"#;
        let mut items = Vec::new();
        detect_unreachable_code("test.rs", content, &mut items);
        // Should detect unreachable code after simple return
        assert!(!items.is_empty());
    }

    #[test]
    fn test_complex_javascript_arrow_functions() {
        let content = r#"
const empty = () => {};
const nonEmpty = () => { return 42; };
const unused_var = 10;
"#;
        let mut items = Vec::new();
        detect_unused_variables("test.js", content, &mut items);
        assert!(items.iter().any(|i| i.name == "unused_var"));
    }

    #[test]
    fn test_complex_go_style_variables() {
        let content = r#"
func test() {
    unused := 10
    used := 20
    fmt.Println(used)
}
"#;
        let mut items = Vec::new();
        detect_unused_variables("test.go", content, &mut items);
        assert!(items.iter().any(|i| i.name == "unused"));
        assert!(!items.iter().any(|i| i.name == "used"));
    }

    #[test]
    fn test_complex_r_style_variables() {
        let content = r#"
test <- function() {
    unused <- 10
    used <- 20
    print(used)
}
"#;
        let mut items = Vec::new();
        detect_unused_variables("test.R", content, &mut items);
        assert!(items.iter().any(|i| i.name == "unused"));
    }

    #[test]
    fn test_edge_case_empty_file() {
        let content = "";
        let mut items = Vec::new();
        detect_unused_variables("test.rs", content, &mut items);
        detect_unreachable_code("test.rs", content, &mut items);
        detect_empty_functions("test.rs", content, &mut items);
        detect_todo_fixme("test.rs", content, &mut items);
        assert!(items.is_empty());
    }

    #[test]
    fn test_edge_case_only_whitespace() {
        let content = "   \n\n   \t\t\n   ";
        let mut items = Vec::new();
        detect_unused_variables("test.rs", content, &mut items);
        detect_unreachable_code("test.rs", content, &mut items);
        detect_empty_functions("test.rs", content, &mut items);
        assert!(items.is_empty());
    }

    #[test]
    fn test_edge_case_very_long_function_name() {
        let content = "fn this_is_a_very_long_function_name_that_should_still_be_detected() {}";
        let mut items = Vec::new();
        detect_empty_functions("test.rs", content, &mut items);
        assert!(items.iter().any(|i| i.name.contains("very_long_function_name")));
    }

    #[test]
    fn test_complex_mixed_language_patterns() {
        // Test file with mixed syntax (shouldn't crash)
        let content = r#"
fn rust_func() {}
function js_func() {}
def python_func():
    pass
proc nim_func() = discard
"#;
        let mut items = Vec::new();
        detect_empty_functions("test.mixed", content, &mut items);
        // Should detect multiple empty functions
        assert!(items.len() >= 2);
    }
}
