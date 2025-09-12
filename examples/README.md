# Code Search Examples

This directory contains example code files in various programming languages to demonstrate and test the code search tool capabilities.

## Example Files

### Rust (`rust_example.rs`)
- Demonstrates struct definitions, implementations, and methods
- Shows error handling with `thiserror`
- Contains test modules and async patterns
- Keywords: `struct`, `impl`, `fn`, `pub`, `Result`, `Error`

### Python (`python_example.py`)
- Object-oriented programming with classes and dataclasses
- Type hints and enums
- Logging and JSON handling
- Keywords: `class`, `def`, `import`, `from`, `@dataclass`, `Enum`

### JavaScript (`javascript_example.js`)
- ES6+ features including classes, arrow functions, and async/await
- Event handling and DOM manipulation
- Map data structure and array methods
- Keywords: `class`, `function`, `async`, `await`, `const`, `let`

### TypeScript (`typescript_example.ts`)
- Interfaces, generics, and type annotations
- Custom error classes and API response types
- Async/await patterns with proper error handling
- Keywords: `interface`, `type`, `class`, `async`, `await`, `enum`

### Go (`go_example.go`)
- Structs, methods, and interfaces
- Error handling and validation
- JSON marshaling and regular expressions
- Keywords: `struct`, `func`, `interface`, `map`, `error`

### Java (`java_example.java`)
- Classes, inheritance, and polymorphism
- Exception handling and collections
- Stream API and lambda expressions
- Keywords: `class`, `public`, `private`, `static`, `interface`, `extends`

## Testing the Code Search Tool

Use these examples to test various search patterns:

### Basic Text Search
```bash
# Search for function definitions
code-search search "function" --extensions js,ts

# Search for class definitions
code-search search "class" --extensions py,js,ts,java

# Search for struct definitions
code-search search "struct" --extensions rs,go
```

### Regex Patterns
```bash
# Find all function definitions
code-search search "fn\\s+\\w+" --extensions rs --line-numbers

# Find all public methods
code-search search "pub\\s+fn" --extensions rs --line-numbers

# Find all async functions
code-search search "async\\s+function" --extensions js,ts --line-numbers

# Find all class methods
code-search search "def\\s+\\w+" --extensions py --line-numbers
```

### Case-Insensitive Search
```bash
# Search for error handling patterns
code-search search "error" --ignore-case --extensions rs,py,js,ts,go,java

# Search for validation patterns
code-search search "validate" --ignore-case --line-numbers
```

### File Filtering
```bash
# List all example files
code-search files

# List only Rust files
code-search files --extensions rs

# List only Python and JavaScript files
code-search files --extensions py,js
```

### JSON Output
```bash
# Get structured output for analysis
code-search search "class" --extensions py,js,ts,java --format json

# Search for imports and exports
code-search search "import|export" --extensions js,ts --format json
```

### Complex Patterns
```bash
# Find all error types
code-search search "Error|Exception" --extensions rs,py,js,ts,go,java --line-numbers

# Find all API endpoints or routes
code-search search "app\\.|router\\.|@app\\.|@router\\." --extensions py,js,ts

# Find all test functions
code-search search "test_|@test|func Test" --extensions py,js,ts,go --line-numbers

# Find all configuration or constants
code-search search "const|CONST|config|Config" --ignore-case --line-numbers
```

## Search Tips

1. **Use regex for complex patterns**: The tool supports full regex syntax
2. **Combine file extensions**: Use comma-separated extensions to search multiple languages
3. **Use line numbers**: Add `--line-numbers` to see context
4. **Case-insensitive search**: Use `--ignore-case` for broader matches
5. **Limit results**: Use `--max-results` to control output size
6. **Exclude directories**: Use `--exclude` to skip build directories

## Example Use Cases

- **Code Review**: Find all TODO comments or error handling patterns
- **Refactoring**: Locate all instances of a function or class
- **Documentation**: Find all public APIs and interfaces
- **Testing**: Locate all test functions and test data
- **Security**: Search for potential security issues or hardcoded secrets
- **Architecture**: Understand code structure and dependencies
