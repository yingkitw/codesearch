# CodeSearch

**Fast, intelligent code search and analysis for 48+ languages.**

Find what you need in seconds: functions, classes, duplicates, dead code, complexity issues.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## What Can You Search & Detect?

### 🔍 **Search Capabilities**

| What | Example | Use Case |
|------|---------|----------|
| **Functions** | `codesearch "fn authenticate" -e rs` | Find where authentication logic lives |
| **Classes** | `codesearch "class User" -e py` | Locate data models |
| **TODO/FIXME** | `codesearch "TODO\|FIXME" .` | Track technical debt |
| **Imports** | `codesearch "^import" -e js` | Understand dependencies |
| **Patterns** | `codesearch "async.*await" --fuzzy` | Find async code (handles typos) |
| **Exact Text** | `codesearch "deprecated_function"` | Find all usages before refactoring |

**Supports:** Regex, fuzzy matching, case-insensitive, multi-language (48+ languages)

### 🔎 **Code Quality Detection**

| What | Command | Value |
|------|---------|-------|
| **Dead Code** | `codesearch deadcode` | Find unused functions, variables, imports, empty functions |
| **Duplicates** | `codesearch duplicates` | Identify copy-paste code (Type-1/2/3 clones) |
| **Complexity** | `codesearch complexity` | Spot overly complex functions (cyclomatic/cognitive) |
| **Circular Deps** | `codesearch circular` | Detect circular dependencies |

### 💡 **Real-World Use Cases**

**Before Refactoring:**
```bash
# Find all usages of old function
codesearch "oldAuthMethod" .
# Result: Found in 12 files, 23 occurrences
```

**Code Review:**
```bash
# Check for technical debt
codesearch deadcode ./src
# Result: 5 unused functions, 12 TODO comments
```

**Understanding Codebase:**
```bash
# Find all authentication-related code
codesearch "auth" -e rs,py --rank
# Result: Ranked by relevance, with line numbers
```

**Quality Check:**
```bash
# Find duplicated code
codesearch duplicates --min-lines 5
# Result: 8 duplicate blocks (90%+ similar)
```

## 🚀 Quick Start

```bash
# Simple search: codesearch <query> [path]
codesearch "function"           # Search current directory
codesearch "TODO" ./src         # Search specific path
codesearch "class" ./src -e py  # Filter by extension

# Fuzzy search (handles typos)
codesearch "usrmngr" . --fuzzy

# Interactive mode
codesearch interactive

# Analysis commands
codesearch analyze              # Codebase metrics
codesearch complexity           # Complexity scores
codesearch metrics              # Comprehensive metrics (all-in-one)
codesearch design-metrics       # Coupling & cohesion
codesearch duplicates           # Find similar code
codesearch deadcode             # Find unused code

# Advanced features
codesearch index                # Build incremental index
codesearch watch                # Watch for file changes

# Graph analysis (6 types)
codesearch ast file.rs          # Abstract Syntax Tree
codesearch cfg file.rs          # Control Flow Graph
codesearch dfg file.rs          # Data Flow Graph
codesearch callgraph .          # Call Graph
codesearch depgraph .           # Dependency Graph
codesearch pdg file.rs          # Program Dependency Graph
codesearch graph-all file.rs    # All graphs

# Other advanced features
codesearch git-history "TODO"   # Search git history
codesearch remote --github "pattern" # Search GitHub
```

## Why CodeSearch?

**Fast & Precise**
- Parallel processing using Rust and rayon
- Exact line numbers with precise matching
- Smart caching for repeated searches

**Language-Aware**
- 48+ languages supported
- Understands functions, classes, imports
- Syntax-specific patterns

**Quality Focused**
- Detect dead code before it ships
- Find duplicates to improve DRY
- Measure complexity to guide refactoring
- Analyze design quality (coupling, cohesion, instability)

**Developer Friendly**
- Interactive REPL mode
- Export to CSV/Markdown
- MCP server for AI agents

**Advanced Capabilities**
- Incremental indexing for large codebases
- Real-time file watching
- Git history search
- Remote repository search (GitHub/GitLab)

**Graph Analysis (6 Types)**
- Abstract Syntax Tree (AST) - code structure
- Control Flow Graph (CFG) - execution paths
- Data Flow Graph (DFG) - variable dependencies
- Call Graph - function relationships
- Dependency Graph - module dependencies
- Program Dependency Graph (PDG) - combined analysis

## Installation

```bash
git clone https://github.com/yingkitw/codesearch.git
cd codesearch
cargo build --release

# Optional: MCP server for AI agents
cargo build --release --features mcp
```

## Common Options

```bash
# Filter by file type
codesearch "pattern" -e rs,py,js

# Exclude directories
codesearch "pattern" -x target,node_modules

# Case-insensitive
codesearch "pattern" -i

# Fuzzy matching (handles typos)
codesearch "patern" --fuzzy

# Rank by relevance
codesearch "pattern" --rank

# Export results
codesearch "pattern" --export csv
```

## 📖 Usage Examples

### Search Patterns

```bash
# codesearch <query> [path] [options]
codesearch "TODO"                       # Search current directory
codesearch "class" ./src                # Search specific folder
codesearch "error" . -e py,js,ts        # Filter by extensions

# Regex patterns
codesearch "fn\\s+\\w+" ./src -e rs     # Rust functions
codesearch "import.*from" . -e ts       # TypeScript imports

# Fuzzy search (handles typos)
codesearch "authetication" . --fuzzy    # Finds "authentication"
```

### Code Analysis

```bash
# Codebase overview
codesearch analyze
# Output: Files, lines, languages, function count, class count

# Complexity analysis
codesearch complexity --threshold 15 --sort
# Output: Files ranked by cyclomatic/cognitive complexity

# Dead code detection (enhanced with 6+ detection types)
codesearch deadcode -e rs,py,js
# Output: Unused variables, unreachable code, empty functions, 
#         TODO/FIXME markers, commented code, unused imports

# Duplicate detection
codesearch duplicates --similarity 0.8
# Output: Similar code blocks that violate DRY
```

### Interactive Mode

```bash
codesearch interactive
```

**Commands:**
- Type any pattern to search
- `/f` - Toggle fuzzy mode
- `/i` - Toggle case insensitivity
- `analyze` - Codebase metrics
- `complexity` - Complexity analysis
- `deadcode` - Dead code detection
- `duplicates` - Find duplicates
- `help` - All commands

### MCP Server (AI Integration)

```bash
# Start MCP server
cargo run --features mcp -- mcp-server

# Agents can call:
# - search_code(query, path, extensions, fuzzy, regex)
# - list_files(path, extensions, exclude)
# - analyze_codebase(path, extensions)
```

## 📊 Output Examples

### Search Results
```
🔍 Search Results for "fn main"
──────────────────────────────

📁 ./src/main.rs (1 match)
  358: fn main() -> Result<(), Box<dyn std::error::Error>> {

📊 Statistics:
  Files searched: 12
  Matches found: 1
  Time: 0.003s
```

### Dead Code Detection
```
🔍 Dead Code Detection
──────────────────────────────

⚠️  Found 12 potential dead code items:

📄 src/example.rs
   [var] L  10: variable 'unused_var' - Variable declared but never used
   [!]   L  25: unreachable - Code after return statement is unreachable
   [∅]   L  42: empty_helper - Empty function with no implementation
   [?]   L  58: // TODO: implement this - TODO marker - incomplete implementation
   [imp] L  72: import 'HashMap' - Imported but never used

📊 Summary:
   • variable: 3
   • unreachable: 2
   • empty: 2
   • todo: 3
   • import: 2
```

### Complexity Analysis
```
📊 Code Complexity Analysis
──────────────────────────────

📁 Files by Complexity (highest first):

  src/search.rs
    Cyclomatic: 45  Cognitive: 38  Lines: 645

  src/analysis.rs
    Cyclomatic: 28  Cognitive: 22  Lines: 378
```

## Supported Languages

48+ languages including: Rust, Python, JavaScript, TypeScript, Go, Java, C/C++, Ruby, PHP, Swift, Kotlin, and more.

See [ARCHITECTURE.md](ARCHITECTURE.md) for technical details.

## License

Apache-2.0 License

---

**Built with Rust** • Fast • Precise • 48+ Languages
