# Common Search Patterns

This file contains useful search patterns for different programming languages and use cases.

## Function Definitions

### Rust
```bash
# All function definitions
codesearch search "fn\\s+\\w+" --extensions rs --line-numbers

# Public functions only
codesearch search "pub\\s+fn" --extensions rs --line-numbers

# Async functions
codesearch search "async\\s+fn" --extensions rs --line-numbers
```

### Python
```bash
# All function definitions
codesearch search "def\\s+\\w+" --extensions py --line-numbers

# Class methods
codesearch search "def\\s+\\w+\\s*\\(" --extensions py --line-numbers

# Private methods (starting with _)
codesearch search "def\\s+_" --extensions py --line-numbers
```

### JavaScript/TypeScript
```bash
# Function declarations
codesearch search "function\\s+\\w+" --extensions js,ts --line-numbers

# Arrow functions
codesearch search "\\w+\\s*=>" --extensions js,ts --line-numbers

# Async functions
codesearch search "async\\s+function|async\\s+\\w+" --extensions js,ts --line-numbers
```

### Go
```bash
# All function definitions
codesearch search "func\\s+\\w+" --extensions go --line-numbers

# Methods (functions with receivers)
codesearch search "func\\s+\\([^)]+\\)\\s+\\w+" --extensions go --line-numbers
```

### Java
```bash
# All method definitions
codesearch search "public\\s+\\w+\\s+\\w+\\s*\\(" --extensions java --line-numbers

# Constructor definitions
codesearch search "public\\s+\\w+\\s*\\(" --extensions java --line-numbers
```

## Class Definitions

### Multi-language
```bash
# All class definitions
codesearch search "class\\s+\\w+" --extensions py,js,ts,java --line-numbers

# Abstract classes (Java)
codesearch search "abstract\\s+class" --extensions java --line-numbers

# Interface definitions
codesearch search "interface\\s+\\w+" --extensions ts,java --line-numbers
```

## Error Handling

### Multi-language
```bash
# All error types and exceptions
codesearch search "Error|Exception" --extensions rs,py,js,ts,go,java --line-numbers

# Try-catch blocks
codesearch search "try\\s*\\{|catch\\s*\\(" --extensions js,ts,java --line-numbers

# Error handling in Rust
codesearch search "Result<|Option<" --extensions rs --line-numbers
```

## Testing

### Multi-language
```bash
# Test functions
codesearch search "test_|@test|func Test|describe|it\\(" --extensions py,js,ts,go --line-numbers

# Test assertions
codesearch search "assert|expect|should" --extensions py,js,ts,go,java --line-numbers

# Test setup/teardown
codesearch search "setUp|tearDown|beforeEach|afterEach" --extensions py,js,ts --line-numbers
```

## Imports and Dependencies

### Multi-language
```bash
# All imports
codesearch search "^import|^from|^use|^#include" --line-numbers

# Specific library imports
codesearch search "import.*react|from.*react" --extensions js,ts --line-numbers

# Standard library imports
codesearch search "import std::|use std::" --extensions rs --line-numbers
```

## Configuration and Constants

### Multi-language
```bash
# Constants and configuration
codesearch search "const|CONST|config|Config|const\\s+\\w+" --ignore-case --line-numbers

# Environment variables
codesearch search "process\\.env|getenv|env::" --extensions js,ts,py,rs --line-numbers

# Configuration files
codesearch search "config\\.|settings\\.|conf\\." --line-numbers
```

## Security Patterns

### Multi-language
```bash
# Potential security issues
codesearch search "password|secret|key|token|auth" --ignore-case --line-numbers

# SQL injection risks
codesearch search "query\\s*\\(|execute\\s*\\(|prepare\\s*\\(" --line-numbers

# Hardcoded credentials
codesearch search "['\"][a-zA-Z0-9+/]{20,}['\"]" --line-numbers
```

## API and Routes

### Multi-language
```bash
# API endpoints
codesearch search "@app\\.|@router\\.|app\\.|router\\." --extensions py,js,ts --line-numbers

# HTTP methods
codesearch search "@GET|@POST|@PUT|@DELETE|@PATCH" --extensions java,py --line-numbers

# Route definitions
codesearch search "route\\s*\\(|get\\s*\\(|post\\s*\\(" --extensions py,js,ts --line-numbers
```

## Documentation

### Multi-language
```bash
# TODO comments
codesearch search "TODO|FIXME|HACK|XXX" --ignore-case --line-numbers

# Documentation comments
codesearch search "^\\s*///|^\\s*#|^\\s*//|^\\s*/\\*" --line-numbers

# Function documentation
codesearch search "///|#\\s*@|/\\*\\*" --line-numbers
```

## Database Patterns

### Multi-language
```bash
# Database queries
codesearch search "SELECT|INSERT|UPDATE|DELETE|CREATE|DROP" --ignore-case --line-numbers

# ORM patterns
codesearch search "findBy|findAll|save|delete|create" --ignore-case --line-numbers

# Migration files
codesearch search "migration|migrate" --ignore-case --line-numbers
```

## Performance Patterns

### Multi-language
```bash
# Caching patterns
codesearch search "cache|Cache|memoize|memo" --ignore-case --line-numbers

# Async/await patterns
codesearch search "async|await|Promise|Future" --ignore-case --line-numbers

# Performance monitoring
codesearch search "performance|benchmark|profile|timing" --ignore-case --line-numbers
```
