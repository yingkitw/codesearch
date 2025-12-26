# Code Search

A fast, intelligent CLI tool for searching and analyzing codebases, built in Rust. Designed as a **code-aware supplement to AI agents and LLMs**, providing precise structural understanding that semantic search cannot deliver.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## 🎯 Why Code Search Matters for AI Agents

**The Problem**: LLMs and RAG systems treat code as text, losing critical structural information which can be provided by search, static analysis of the code.

**The Solution**: Code Search provides **structured, precise code intelligence** that agents can trust:

```
┌─────────────────────────────────────────────────────────────────┐
│                     AI AGENT / LLM                              │
│  "I need to understand the authentication module"               │
└──────────────────────────┬──────────────────────────────────────┘
                           │
         ┌─────────────────┴─────────────────┐
         │                                   │
    ┌────▼────┐                        ┌─────▼─────┐
    │   RAG   │                        │ CodeSearch│
    │ Semantic│                        │ Structural│
    └────┬────┘                        └─────┬─────┘
         │                                   │
    "Files about                      "auth.rs: L15-45
     authentication"                   fn authenticate()
     (fuzzy, chunked)                  fn verify_token()
                                       3 callers, 2 deps"
                                       (precise, complete)
```

## 🧠 Key Capabilities

### 1. **Precise Pattern Matching** (Not Semantic Guessing)

| What You Need | RAG/Embeddings | Code Search |
|---------------|----------------|-------------|
| Find `fn authenticate` | Returns similar functions | Returns exact function + line number |
| Find all TODO comments | Misses non-standard formats | Regex: `TODO\|FIXME\|HACK` catches all |
| Find unused imports | Cannot detect | Analyzes actual usage |
| Rename `oldFunc` → `newFunc` | Suggests similar names | Finds every exact occurrence |

### 2. **Language-Aware Intelligence** (48 Languages)

```bash
# Each language has tailored patterns
codesearch "fn\\s+\\w+" -e rs      # Rust functions
codesearch "def\\s+\\w+" -e py      # Python functions  
codesearch "async\\s+function" -e js # JS async functions
```

**Understands:**
- Function definitions, class structures, imports
- Comment patterns (single-line, multi-line, doc comments)
- Language-specific syntax (traits, interfaces, decorators)

### 3. **Code Quality Analysis**

| Analysis | What It Finds |
|----------|---------------|
| **Complexity** | Cyclomatic & cognitive complexity scores |
| **Dead Code** | Unused imports, functions, classes |
| **Duplicates** | Similar code blocks (DRY violations) |

### 4. **MCP Server for Agent Integration**

Exposes code intelligence as tools that AI agents can call:

```bash
cargo run --features mcp -- mcp-server
```

**Available Tools:**
- `search_code` - Find patterns with fuzzy/regex support
- `list_files` - Enumerate codebase with filters
- `analyze_codebase` - Get metrics and statistics

## 🔄 How It Complements RAG & LLMs

| Aspect | RAG Alone | + Code Search |
|--------|-----------|---------------|
| **Find function** | "Similar to auth..." | Exact: `auth.rs:L42` |
| **Count usages** | "Mentioned several times" | Precise: "Called 7 times in 3 files" |
| **Find all usages** | Suggests changes | Validates all occurrences found |
| **Dead code** | Cannot detect | Lists unused with line numbers |
| **Complexity** | No metrics | Cyclomatic score: 15 |

### The Hybrid Approach

```
┌────────────────────────────────────────────────────────────────────┐
│  User: "Help me understand and improve the auth module"           │
└────────────────────────────────────────────────────────────────────┘
                                │
                ┌───────────────┼───────────────┐
                ▼               ▼               ▼
         ┌──────────┐    ┌──────────┐    ┌──────────┐
         │   RAG    │    │ CodeSearch│   │  LLM     │
         │ Semantic │    │ Structural│   │ Reasoning│
         └────┬─────┘    └─────┬────┘    └────┬─────┘
              │                │              │
      "Auth handles       "auth.rs:         "Based on the
       user login,         fn login() L12    structure, I
       sessions..."        fn verify() L45   recommend..."
                           complexity: 18
                           dead code: 2
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
codesearch duplicates           # Find similar code
codesearch deadcode             # Find unused code
```

## ✨ Features

- **Fast regex search** with exact line-level precision
- **Fuzzy matching** for typo tolerance
- **48 language support** with syntax awareness
- **Interactive REPL** for exploratory analysis
- **Code metrics** - complexity, duplication, dead code
- **Export** results to CSV or Markdown
- **MCP server** for AI agent integration
- **Parallel processing** for large codebases

## 🏗️ Installation

```bash
git clone https://github.com/yingkitw/codesearch.git
cd codesearch
cargo build --release

# With MCP server support for AI agents
cargo build --release --features mcp
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

# Dead code detection
codesearch deadcode -e rs,py,js
# Output: Unused imports, functions, classes

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

⚠️  Found 5 potential dead code items:

📄 examples/deadcode_demo.rs
   📥 L   4: import 'HashMap' - Imported but never used
   📥 L   6: import 'Write' - Imported but never used

📊 Summary:
   • import: 5
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

## 🔧 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│  main.rs (358 LOC) - Argument parsing, command routing       │
│  interactive.rs (350 LOC) - REPL interface                   │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      Core Engine                             │
│  search.rs (645 LOC) - Pattern matching, fuzzy, ranking      │
│  language.rs (509 LOC) - 48 language definitions             │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Analysis Layer                           │
│  analysis.rs (378 LOC) - Codebase metrics                   │
│  complexity.rs (308 LOC) - Cyclomatic/cognitive complexity   │
│  deadcode.rs (373 LOC) - Unused code detection              │
│  duplicates.rs (196 LOC) - Similar code detection           │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    Integration Layer                         │
│  mcp_server.rs (295 LOC) - MCP protocol for AI agents       │
│  export.rs (185 LOC) - CSV/Markdown output                  │
│  cache.rs (127 LOC) - Result caching                        │
└─────────────────────────────────────────────────────────────┘
```

**11 modules, ~3,800 lines of Rust code**

## 🧪 Testing

```bash
# Run all tests (84 total)
cargo test --features mcp

# Unit tests: 35 (core functionality)
# Integration tests: 26 (CLI commands)  
# MCP tests: 23 (server tools)
```

## ⚡ Performance

- **10x faster** than grep for complex patterns
- **Parallel processing** with rayon
- **Memory efficient** streaming for large files
- **Compiled regex** patterns cached
- **Smart defaults** exclude build directories

## 📚 Documentation

- [README.md](README.md) - This guide
- [ARCHITECTURE.md](ARCHITECTURE.md) - System design
- [TODO.md](TODO.md) - Roadmap
- [examples/](examples/) - Code samples with dead code demos

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Test: `cargo test --features mcp`
4. Submit a pull request

## 📄 License

Apache-2.0 License

---

**Built with ❤️ in Rust** | **Precise** | **Fast** | **Agent-Ready**

*"RAG tells you about code. Code Search shows you the code."*
