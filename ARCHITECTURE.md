# Architecture

## Overview

Code Search is a CLI tool built in Rust that provides fast, intelligent code searching capabilities. The architecture is designed for performance, maintainability, and extensibility.

## System Architecture

```mermaid
graph TB
    CLI[Cli Interface] --> Parser[Command Parser]
    Parser --> Search[Search Engine]
    Parser --> Analyze[Analysis Engine]
    Parser --> MCP[MCP Server]
    
    Search --> Cache[Search Cache]
    Search --> Parallel[Parallel Processor]
    Search --> Regex[Regex Engine]
    Search --> Fuzzy[Fuzzy Matcher]
    
    Analyze --> FileWalker[File Walker]
    Analyze --> Metrics[Metrics Calculator]
    
    Cache --> DashMap[Thread-Safe Cache]
    Parallel --> Rayon[Rayon Thread Pool]
    
    FileWalker --> WalkDir[WalkDir]
    
    MCP --> Tools[MCP Tools]
    Tools --> Search
    Tools --> Analyze
```

## Module Structure

```mermaid
graph TB
    Main[main.rs<br/>CLI & Entry Point] --> Search[search.rs<br/>Core Search]
    Main --> Analysis[analysis.rs<br/>Codebase Analysis]
    Main --> Complexity[complexity.rs<br/>Complexity Metrics]
    Main --> Deadcode[deadcode.rs<br/>Dead Code Detection]
    Main --> Duplicates[duplicates.rs<br/>Duplication Detection]
    Main --> Favorites[favorites.rs<br/>Favorites & History]
    Main --> Export[export.rs<br/>Export CSV/MD]
    Main --> MCP[mcp_server.rs<br/>MCP Integration]
    
    Search --> Cache[cache.rs<br/>Caching System]
    Search --> Types[types.rs<br/>Data Structures]
    Search --> Language[language.rs<br/>48 Languages]
    
    Analysis --> Language
    Analysis --> Search
    Complexity --> Language
    Complexity --> Search
    
    Main --> Config[config.rs<br/>Configuration]
    Main --> Theme[theme.rs<br/>6 Themes]
    
    MCP --> Search
    MCP --> Analysis
```

### Module Descriptions (13 modules, ~4500 LOC total)

| Module | Lines | Description |
|--------|-------|-------------|
| `main.rs` | 699 | CLI entry point, interactive mode |
| **`deadcode/`** | **685** | **Enhanced dead code detection (modularized)** |
| ├─ `mod.rs` | 200 | Main detection logic and public API |
| ├─ `detectors.rs` | 340 | Individual detection functions (6+ types) |
| ├─ `helpers.rs` | 130 | Utility functions and validators |
| └─ `types.rs` | 15 | Data structures (DeadCodeItem) |
| `search.rs` | 645 | Core search engine, parallel processing |
| `language.rs` | 510 | 48 language definitions with patterns |
| `analysis.rs` | 418 | Codebase metrics, refactoring suggestions |
| `mcp_server.rs` | 375 | MCP server integration |
| `complexity.rs` | 308 | Cyclomatic & cognitive complexity |
| `duplicates.rs` | 196 | Code duplication detection |
| `favorites.rs` | 199 | Favorites and search history |
| `export.rs` | 185 | CSV and Markdown export |
| `config.rs` | 183 | Configuration file support |
| `theme.rs` | 179 | 6 output themes |
| `parser.rs` | 176 | Code parsing utilities |
| `cache.rs` | 125 | Thread-safe search cache |
| `types.rs` | 112 | Shared data structures |

## Data Flow

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant Parser
    participant SearchEngine
    participant Cache
    participant FileSystem
    participant ParallelProcessor
    
    User->>CLI: Search Query
    CLI->>Parser: Parse Command
    Parser->>SearchEngine: Execute Search
    SearchEngine->>Cache: Check Cache
    alt Cache Hit
        Cache-->>SearchEngine: Return Cached Results
    else Cache Miss
        SearchEngine->>FileSystem: Walk Directory
        FileSystem-->>SearchEngine: File List
        SearchEngine->>ParallelProcessor: Process Files
        ParallelProcessor->>FileSystem: Read Files
        FileSystem-->>ParallelProcessor: File Content
        ParallelProcessor->>SearchEngine: Matches
        SearchEngine->>Cache: Store Results
    end
    SearchEngine-->>CLI: Search Results
    CLI-->>User: Formatted Output
```

## Core Components

### 1. CLI Interface (`main.rs`)

**Responsibilities:**
- Command-line argument parsing using `clap`
- Command routing to appropriate handlers
- Output formatting (text/JSON)
- User interaction (interactive mode, favorites)

**Key Structures:**
- `Cli`: Main CLI parser structure
- `Commands`: Enum of available commands
- `SearchResult`, `Match`, `FileInfo`: Data structures for results

### 2. Search Engine (`search_code`)

**Responsibilities:**
- Pattern matching (regex, fuzzy, exact)
- Parallel file processing
- Relevance scoring
- Result ranking and filtering

**Key Functions:**
- `search_code()`: Main search entry point
- `search_in_file_parallel()`: Parallel file search
- `calculate_relevance_score()`: Relevance calculation
- `enhance_query_semantically()`: Query enhancement

**Performance Features:**
- Parallel processing with `rayon`
- Thread-safe caching with `DashMap`
- Streaming file reading for large files

### 3. Caching System (`SearchCache`)

**Responsibilities:**
- Query result caching
- File modification tracking
- Cache invalidation
- Performance optimization

**Implementation:**
- Uses `DashMap` for thread-safe concurrent access
- Tracks file modification times
- Cache key includes: query, path, extensions, fuzzy flag

### 4. Analysis Engine (`analyze_codebase`, `suggest_refactoring`)

**Responsibilities:**
- Codebase metrics calculation
- Code quality analysis
- Refactoring suggestions
- File statistics

**Key Functions:**
- `analyze_codebase()`: Overall codebase analysis
- `analyze_file_for_refactoring()`: File-level analysis
- `suggest_refactoring()`: Refactoring recommendations

### 5. Dead Code Detection Engine (`deadcode/`)

**Modular Structure** (following SoC and KISS principles):

The dead code detection module has been refactored into focused sub-modules:

```
deadcode/
├── mod.rs          # Main detection logic, orchestration, output formatting
├── detectors.rs    # Individual detection functions (6+ detection types)
├── helpers.rs      # Utility functions, validators, string operations
└── types.rs        # Data structures (DeadCodeItem)
```

**Responsibilities:**
- Detect unused variables and constants
- Identify unreachable code after return statements
- Find empty functions (supports brace-based and indentation-based languages)
- Flag TODO/FIXME/HACK/XXX/BUG markers
- Detect commented-out code
- Identify unused imports

**Key Functions:**

*In `mod.rs`:*
- `detect_dead_code()`: CLI entry point with formatted output
- `find_dead_code()`: Main detection orchestration
- `print_dead_code_results()`: Result formatting

*In `detectors.rs`:*
- `detect_unused_variables()`: Variable usage analysis
- `detect_unreachable_code()`: Control flow analysis
- `detect_empty_functions()`: Empty function detection (multi-language)
- `detect_todo_fixme()`: Marker detection
- `detect_dead_code_patterns()`: Pattern-based detection

*In `helpers.rs`:*
- `is_special_function()`: Function name validation
- `is_commented_out_code()`: Code comment detection
- `extract_import_name()`: Import statement parsing
- `truncate_string()`: String formatting

**Detection Types:**
- **Unused Variables**: Detects `let`, `const`, `var`, `:=`, `<-` patterns
- **Unreachable Code**: Identifies code after return statements
- **Empty Functions**: Handles both `{}` and `:` (Python) syntax
- **TODO Markers**: Finds TODO, FIXME, HACK, XXX, BUG comments
- **Commented Code**: Detects commented-out function/variable declarations
- **Unused Imports**: Tracks import usage across files

**Benefits of Modularization:**
- **Maintainability**: Each sub-module has a single, clear responsibility
- **Testability**: Tests are co-located with their respective modules
- **Extensibility**: New detection types can be added to `detectors.rs` without affecting other code
- **Readability**: Smaller files (~130-340 LOC each) are easier to understand

### 6. MCP Server (`mcp_server.rs`)

**Responsibilities:**
- Expose code search as MCP tools
- JSON schema generation
- Tool routing and handling
- Integration with AI assistants

**Tools Exposed:**
- `search_code`: Search functionality
- `list_files`: File listing
- `analyze_codebase`: Codebase analysis
- `suggest_refactoring`: Refactoring suggestions

## Data Structures

### SearchResult
```rust
pub struct SearchResult {
    pub file: String,
    pub matches: Vec<Match>,
    pub line_count: usize,
    pub relevance_score: f64,
}
```

### Match
```rust
pub struct Match {
    pub line_number: usize,
    pub content: String,
    pub context: Option<String>,
}
```

### FileInfo
```rust
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub lines: usize,
}
```

### SearchCache
```rust
struct SearchCache {
    cache: DashMap<String, (Vec<SearchResult>, SystemTime)>,
    file_timestamps: DashMap<String, SystemTime>,
}
```

## Performance Optimizations

### Parallel Processing
- Uses `rayon` for parallel file processing
- Automatically scales to available CPU cores
- Thread-safe operations throughout

### Caching Strategy
- Query-based caching with file modification tracking
- Automatic cache invalidation on file changes
- Memory-efficient storage

### Memory Management
- Streaming file reading (doesn't load entire files)
- Efficient data structures (`DashMap`, `ahash`)
- Lazy evaluation where possible

## Error Handling

- Uses `anyhow` for error propagation
- Uses `thiserror` for custom error types
- Graceful handling of file access errors
- Continues processing on individual file errors

## Testing Architecture

### Unit Tests (49+ tests across modules)
- Each module contains its own unit tests
- Located in respective module files (`search.rs`, `analysis.rs`, etc.)
- Test individual functions in isolation
- Use temporary directories for file operations

### Integration Tests
- Located in `tests/integration_tests.rs` (26 tests)
- Test CLI commands end-to-end
- Verify output format and behavior

### Dead Code Detection Tests (11 tests)
- `test_detect_unused_variables`: Variable usage tracking
- `test_detect_unreachable_code`: Control flow analysis
- `test_detect_empty_functions`: Multi-language empty function detection
- `test_detect_todo_fixme`: Marker detection
- `test_multi_language_support`: Python/Rust compatibility
- Additional tests for edge cases and patterns

## Dependencies

### Core Dependencies
- `clap`: CLI argument parsing
- `regex`: Pattern matching
- `walkdir`: Directory traversal
- `serde`/`serde_json`: Serialization

### Performance Dependencies
- `rayon`: Parallel processing
- `dashmap`: Thread-safe hash maps
- `ahash`: Fast hashing
- `fuzzy-matcher`: Fuzzy search

### Optional Dependencies
- `rmcp`: MCP server support
- `tokio`: Async runtime (for MCP)
- `schemars`: JSON schema generation (for MCP)

## Extension Points

### Adding New Commands
1. Add variant to `Commands` enum
2. Implement handler function
3. Add CLI argument parsing

### Adding New Search Features
1. Extend `search_code` function
2. Add new scoring factors to `calculate_relevance_score`
3. Update cache key generation if needed

### Adding MCP Tools
1. Define parameter struct with `JsonSchema`
2. Implement tool function in `mcp_server.rs`
3. Register with `tool_router`

## Build Configuration

- **Rust Edition**: 2024
- **Default Features**: None (minimal dependencies)
- **Optional Features**: `mcp` (MCP server support)
- **Target**: Native binary (CLI-only, no WASM)

## Future Architecture Considerations

- **Separate Crates**: Consider splitting into workspace crates for very large projects
- **Plugin System**: Allow external search strategies and custom analyzers
- **Incremental Indexing**: Add indexing for very large codebases (1M+ files)
- **AST-Based Analysis**: Move beyond regex to true syntax tree analysis
- **Remote Search**: Add support for searching in remote repositories

