# Common Search Patterns

This file contains useful search patterns for different programming languages and use cases.

## Function Definitions

### Rust
```bash
# All function definitions
code-search search "fn\\s+\\w+" --extensions rs --line-numbers

# Public functions only
code-search search "pub\\s+fn" --extensions rs --line-numbers

# Async functions
code-search search "async\\s+fn" --extensions rs --line-numbers
```

### Python
```bash
# All function definitions
code-search search "def\\s+\\w+" --extensions py --line-numbers

# Class methods
code-search search "def\\s+\\w+\\s*\\(" --extensions py --line-numbers

# Private methods (starting with _)
code-search search "def\\s+_" --extensions py --line-numbers
```

### JavaScript/TypeScript
```bash
# Function declarations
code-search search "function\\s+\\w+" --extensions js,ts --line-numbers

# Arrow functions
code-search search "\\w+\\s*=>" --extensions js,ts --line-numbers

# Async functions
code-search search "async\\s+function|async\\s+\\w+" --extensions js,ts --line-numbers
```

### Go
```bash
# All function definitions
code-search search "func\\s+\\w+" --extensions go --line-numbers

# Methods (functions with receivers)
code-search search "func\\s+\\([^)]+\\)\\s+\\w+" --extensions go --line-numbers
```

### Java
```bash
# All method definitions
code-search search "public\\s+\\w+\\s+\\w+\\s*\\(" --extensions java --line-numbers

# Constructor definitions
code-search search "public\\s+\\w+\\s*\\(" --extensions java --line-numbers
```

## Class Definitions

### Multi-language
```bash
# All class definitions
code-search search "class\\s+\\w+" --extensions py,js,ts,java --line-numbers

# Abstract classes (Java)
code-search search "abstract\\s+class" --extensions java --line-numbers

# Interface definitions
code-search search "interface\\s+\\w+" --extensions ts,java --line-numbers
```

## Error Handling

### Multi-language
```bash
# All error types and exceptions
code-search search "Error|Exception" --extensions rs,py,js,ts,go,java --line-numbers

# Try-catch blocks
code-search search "try\\s*\\{|catch\\s*\\(" --extensions js,ts,java --line-numbers

# Error handling in Rust
code-search search "Result<|Option<" --extensions rs --line-numbers
```

## Testing

### Multi-language
```bash
# Test functions
code-search search "test_|@test|func Test|describe|it\\(" --extensions py,js,ts,go --line-numbers

# Test assertions
code-search search "assert|expect|should" --extensions py,js,ts,go,java --line-numbers

# Test setup/teardown
code-search search "setUp|tearDown|beforeEach|afterEach" --extensions py,js,ts --line-numbers
```

## Imports and Dependencies

### Multi-language
```bash
# All imports
code-search search "^import|^from|^use|^#include" --line-numbers

# Specific library imports
code-search search "import.*react|from.*react" --extensions js,ts --line-numbers

# Standard library imports
code-search search "import std::|use std::" --extensions rs --line-numbers
```

## Configuration and Constants

### Multi-language
```bash
# Constants and configuration
code-search search "const|CONST|config|Config|const\\s+\\w+" --ignore-case --line-numbers

# Environment variables
code-search search "process\\.env|getenv|env::" --extensions js,ts,py,rs --line-numbers

# Configuration files
code-search search "config\\.|settings\\.|conf\\." --line-numbers
```

## Security Patterns

### Multi-language
```bash
# Potential security issues
code-search search "password|secret|key|token|auth" --ignore-case --line-numbers

# SQL injection risks
code-search search "query\\s*\\(|execute\\s*\\(|prepare\\s*\\(" --line-numbers

# Hardcoded credentials
code-search search "['\"][a-zA-Z0-9+/]{20,}['\"]" --line-numbers
```

## API and Routes

### Multi-language
```bash
# API endpoints
code-search search "@app\\.|@router\\.|app\\.|router\\." --extensions py,js,ts --line-numbers

# HTTP methods
code-search search "@GET|@POST|@PUT|@DELETE|@PATCH" --extensions java,py --line-numbers

# Route definitions
code-search search "route\\s*\\(|get\\s*\\(|post\\s*\\(" --extensions py,js,ts --line-numbers
```

## Documentation

### Multi-language
```bash
# TODO comments
code-search search "TODO|FIXME|HACK|XXX" --ignore-case --line-numbers

# Documentation comments
code-search search "^\\s*///|^\\s*#|^\\s*//|^\\s*/\\*" --line-numbers

# Function documentation
code-search search "///|#\\s*@|/\\*\\*" --line-numbers
```

## Database Patterns

### Multi-language
```bash
# Database queries
code-search search "SELECT|INSERT|UPDATE|DELETE|CREATE|DROP" --ignore-case --line-numbers

# ORM patterns
code-search search "findBy|findAll|save|delete|create" --ignore-case --line-numbers

# Migration files
code-search search "migration|migrate" --ignore-case --line-numbers
```

## Performance Patterns

### Multi-language
```bash
# Caching patterns
code-search search "cache|Cache|memoize|memo" --ignore-case --line-numbers

# Async/await patterns
code-search search "async|await|Promise|Future" --ignore-case --line-numbers

# Performance monitoring
code-search search "performance|benchmark|profile|timing" --ignore-case --line-numbers
```
