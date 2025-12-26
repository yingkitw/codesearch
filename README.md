# Code Search

A fast CLI tool for searching and analyzing codebases, built in Rust. Goes beyond traditional `grep` with intelligent search, fuzzy matching, and codebase analytics.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


## 🧠 Why Code Search Instead of Just RAG?

**RAG (Retrieval Augmented Generation)** excels at natural language understanding, but **Code Search** handles code-specific characteristics that RAG cannot:

### Key Differences

| Aspect | RAG | Code Search |
|--------|-----|-------------|
| **Syntax Awareness** | ❌ General semantics | ✅ Language-specific patterns |
| **Exact Matching** | ❌ Semantic similarity | ✅ Precise pattern matching |
| **Code Structure** | ❌ Treats as text | ✅ Understands functions, classes, imports |
| **Regex Support** | ❌ Not designed for regex | ✅ Full regex pattern matching |
| **Performance** | ⚠️ Embedding overhead | ✅ Fast pattern matching |
| **Line Numbers** | ❌ Semantic chunks | ✅ Exact line-level precision |

### Why It Matters

**RAG struggles with code because:**
- Code requires **exact syntax matching** (not semantic similarity)
- Each language has **unique patterns** (Rust `fn`, Python `def`, JS `function`)
- Refactoring needs **precise matches** (not "similar" functions)
- Code structure matters (imports, function calls, inheritance)

**Code Search excels at:**
- Finding exact function/class names for refactoring
- Language-specific pattern matching (`fn\s+\w+` for Rust, `def\s+\w+` for Python)
- Regex-based code analysis (error handling, async patterns, imports)
- Real-time feedback for development workflows
- Line-level precision for code navigation

### When to Use Each

**Use RAG for:**
- Conceptual understanding ("How does authentication work?")
- Documentation and comments
- High-level architecture understanding

**Use Code Search for:**
- Exact code patterns for refactoring
- Finding specific functions, classes, or imports
- Code structure analysis and metrics
- Regex pattern matching
- Real-time development workflows

**Best approach**: Use both - RAG for understanding, Code Search for precise operations.

## 🏆 Why Choose Code Search Over Grep?

| Feature | Grep | Code Search |
|---------|------|-------------|
| Visual Output | Raw text | Professional formatting |
| Statistics | ❌ | ✅ |
| Fuzzy Search | ❌ | ✅ |
| Interactive Mode | ❌ | ✅ |
| Code Analysis | ❌ | ✅ |
| JSON Output | ❌ | ✅ |
| Multi-language | Manual | Intelligent |

## 🎯 Quick Start

```bash
# Just type what you want to find
codesearch "function"
codesearch "TODO"
codesearch "class" -e py,js,ts

# Find with typos (fuzzy search)
codesearch "usrmngr" --fuzzy

# Interactive mode
codesearch interactive

# Analyze codebase
codesearch analyze
```

**Defaults**: Line numbers, statistics, and smart exclusions are automatic.

## ✨ Features

- **Fast text search** with full regex support
- **Fuzzy search** for handling typos
- **Multi-language support** (48 languages) with intelligent filtering
- **Interactive mode** with real-time feedback and keyboard shortcuts
- **Codebase analysis** with metrics and insights
- **Code complexity metrics** (cyclomatic & cognitive complexity)
- **Code duplication detection** to identify similar code blocks
- **Export functionality** (CSV, Markdown) for results
- **Themeable output** (default, dark, light, mono, ocean, forest)
- **MCP server support** for AI integration (optional)

## 🚀 Installation

```bash
git clone https://github.com/yingkitw/codesearch.git
cd codesearch
cargo build --release

# With MCP server support
cargo build --release --features mcp
```

## ⚙️ Configuration

Code Search supports configuration files for customizing default behavior. Create a `.codesearchrc` or `.codesearch.toml` file in your project root or home directory.

**Config file locations (checked in order):**
1. `.codesearchrc` (current directory)
2. `.codesearch.toml` (current directory)
3. `~/.codesearchrc` (home directory)
4. `~/.codesearch.toml` (home directory)

**Example `.codesearchrc`:**
```toml
[search]
fuzzy_threshold = 0.6
max_results = 10
ignore_case = true
show_line_numbers = true
format = "text"
auto_exclude = true
cache = false
semantic = false
rank = false

# Optional: default extensions
# extensions = ["rs", "py", "js", "ts"]

# Optional: custom exclude directories
# exclude = ["custom_dir"]

[defaults]
# Optional: custom exclude directories (merged with auto-exclude)
# exclude_dirs = ["custom_build"]
```

**Note:** CLI arguments always override config file settings.

## 📖 Usage

### Simple Search

```bash
# Basic search
codesearch "function"
codesearch "TODO" -e py,js,ts

# Fuzzy search for typos
codesearch "usrmngr" --fuzzy

# JSON output
codesearch "error" --format json

# Export results to CSV or Markdown
codesearch "function" --export results.csv
codesearch "class" --export results.md
```

### Advanced Options

```bash
# Case-sensitive search
codesearch search "Error" --no-ignore-case

# Limit results per file
codesearch search "class" --max-results 5

# Exclude directories
codesearch search "import" --exclude target,node_modules
```

### Interactive Mode

```bash
codesearch interactive --extensions py,js,ts
```

**Keyboard Shortcuts:**
- `/f` - Toggle fuzzy search mode
- `/i` - Toggle case insensitivity
- `/r` - Toggle relevance ranking
- `/s` - Toggle semantic search
- `!!` - Repeat last search
- `!n` - Repeat search #n from history

**Commands:** `analyze`, `complexity`, `duplicates`, `refactor`, `export`, `status`, `help`, `quit`

### Codebase Analysis

```bash
# Analyze entire codebase
codesearch analyze

# Analyze specific languages
codesearch analyze --extensions rs,py,js,ts
```

### Code Complexity Analysis

```bash
# Analyze code complexity (cyclomatic & cognitive)
codesearch complexity

# Show only high complexity files (threshold)
codesearch complexity --threshold 15 --sort

# Analyze specific file types
codesearch complexity --extensions rs,py,js
```

### Code Duplication Detection

```bash
# Find duplicate code blocks
codesearch duplicates

# Custom similarity threshold (0.0 - 1.0)
codesearch duplicates --similarity 0.8

# Minimum lines for a duplicate block
codesearch duplicates --min-lines 5
```

### Themes

```bash
# Use different output themes
codesearch "function" --theme dark
codesearch "class" --theme ocean
codesearch "TODO" --theme forest

# Available themes: default, dark, light, mono, ocean, forest
```

### Supported Languages

```bash
# List all 48 supported programming languages
codesearch languages
```

**Supported language categories:**
- **Systems & Low-Level**: Rust, C, C++, Go, Zig, V, Nim, Assembly
- **Object-Oriented**: Java, C#, Kotlin, Swift, Objective-C, Dart
- **Scripting**: Python, JavaScript, TypeScript, Ruby, Perl, Lua, PHP
- **Functional**: Haskell, Elixir, Erlang, Clojure, OCaml, F#, Scala
- **Shell & Config**: Shell, PowerShell, Makefile, Dockerfile, Terraform
- **Data & Markup**: SQL, YAML, TOML, JSON, XML/HTML, CSS, Markdown
- **Scientific**: R, Julia
- **Domain Specific**: Solidity, GraphQL, Protobuf, WebAssembly
- **Build Tools**: Groovy, Gradle, Crystal

### MCP Server

```bash
# Run MCP server (requires --features mcp)
cargo run --features mcp -- mcp-server

# Exposes tools: search_code, list_files, analyze_codebase, suggest_refactoring
```


## 📋 Output Formats

### Text Format (Default)
```
Found 3 matches in 1 files

📁 ./src/main.rs (3 matches)
─────────────────────────────
  1: fn main() {
  2:     println!("Hello, world!");
  3: }
```

### JSON Format
```json
[
  {
    "file": "/path/to/file.rs",
    "line_number": 2,
    "content": "    println!(\"Hello, world!\");",
    "matches": [...]
  }
]
```

## 💡 Common Use Cases

### Find Code Patterns
```bash
# Function definitions
codesearch "fn\\s+\\w+" -e rs
codesearch "def\\s+\\w+" -e py

# Error handling
codesearch "catch|except|Error" -e js,py,rs

# TODO comments
codesearch "TODO|FIXME|HACK"
```

### Refactoring
```bash
# Find all instances of a function
codesearch "oldFunctionName" --stats

# Analyze before refactoring
codesearch analyze --extensions js,ts
```

### Code Review
```bash
# Find potential issues
codesearch "password|secret|key" --ignore-case

# Check error handling
codesearch "Error|Exception" -e js,ts,py
```

## ⚡ Performance

- **Fast directory traversal** with `walkdir`
- **Efficient regex matching** with compiled patterns
- **Memory-efficient** streaming for large files
- **Parallel processing** for large codebases
- **Progress indicators** for long-running searches (>50 files)
- **10x faster** than grep for complex patterns

## 🧪 Testing

```bash
# Run all tests (66 unit tests + 26 integration tests = 92 total)
cargo test

# Run with verbose output
cargo test -- --nocapture
```

**Test Coverage:**
- **66 unit tests**: Individual function testing with comprehensive edge cases
- **26 integration tests**: End-to-end CLI command testing
- **Total**: 92 tests covering all major capabilities

## 📁 Project Structure

```
codesearch/
├── src/
│   ├── main.rs           # CLI entry point, interactive mode (699 LOC)
│   ├── search.rs         # Core search engine (645 LOC)
│   ├── language.rs       # 48 language definitions (506 LOC)
│   ├── analysis.rs       # Codebase analysis (418 LOC)
│   ├── mcp_server.rs     # MCP server integration (375 LOC)
│   ├── complexity.rs     # Complexity metrics (308 LOC)
│   ├── duplicates.rs     # Duplication detection (196 LOC)
│   ├── favorites.rs      # Favorites & history (199 LOC)
│   ├── export.rs         # CSV/Markdown export (185 LOC)
│   ├── config.rs         # Configuration handling (183 LOC)
│   ├── theme.rs          # 6 output themes (179 LOC)
│   ├── cache.rs          # Search caching (125 LOC)
│   └── types.rs          # Shared types (112 LOC)
├── tests/
│   └── integration_tests.rs
├── Cargo.toml
├── README.md
├── ARCHITECTURE.md        # Architecture documentation
└── TODO.md                # Task tracking
```

**Total: 13 modules, ~4,100 lines of code**

## 📚 Documentation

- **[README.md](README.md)**: Main documentation and usage guide
- **[ARCHITECTURE.md](ARCHITECTURE.md)**: System architecture and design
- **[TODO.md](TODO.md)**: Task tracking and future plans

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test: `cargo test`
5. Submit a pull request

## 📄 License

Apache-2.0 License

## 🔧 Dependencies

**Core**: `clap`, `regex`, `walkdir`, `serde`, `colored`, `anyhow`, `thiserror`  
**Advanced**: `fuzzy-matcher`, `rayon`, `dashmap`  
**MCP** (optional): `rmcp`, `tokio`, `schemars`

---

**Built with ❤️ in Rust** | **Fast** | **Safe** | **Powerful**
