# Graph Analysis Implementation

## Overview

Comprehensive implementation of 6 graph analysis types for code understanding and optimization.

## Implemented Graph Types

### 1. Abstract Syntax Tree (AST)
**Module**: `src/ast.rs` (420 LOC)

**What it represents**: Hierarchical structure of source code parsed into a tree where each node represents a construct (expressions, statements, functions, variables).

**Capabilities**:
- Parse Rust, Python, JavaScript using tree-sitter
- Extract functions with parameters, return types, visibility
- Extract classes with methods and fields
- Extract imports and variables
- Detect async functions and public/private visibility
- Find function call sites

**CLI Command**:
```bash
codesearch ast <file> --format json
codesearch ast . -e rs,py,js
```

**Use Cases**:
- Syntax checking and analysis
- Refactoring tools
- Code similarity analysis
- Static analysis for vulnerabilities

**Tests**: 2 unit tests

---

### 2. Control Flow Graph (CFG)
**Module**: `src/cfg.rs` (350 LOC)

**What it represents**: Flow of control within a program. Each node is a basic block (sequence of instructions with no jumps), edges represent control flow paths.

**Capabilities**:
- Build CFG from source code
- Identify basic blocks (entry, normal, branch, loop, exit, return)
- Detect unreachable code
- Find loops and loop bodies
- Calculate cyclomatic complexity
- Export to DOT format for visualization

**CLI Command**:
```bash
codesearch cfg <file> --format dot --export cfg.dot
codesearch cfg <file> --format json
```

**Use Cases**:
- Analyzing loops and conditions
- Compiler optimization
- Detecting unreachable code or infinite loops
- Security analysis for vulnerabilities

**Tests**: 4 unit tests

---

### 3. Data Flow Graph (DFG)
**Module**: `src/dfg.rs` (380 LOC)

**What it represents**: Flow of data between operations or variables. Nodes represent operations/variables, edges show data flow.

**Capabilities**:
- Track variable definitions and uses
- Identify data dependencies
- Calculate variable lifetimes
- Find unused variables
- Detect redundant computations
- Parameter and return value tracking

**CLI Command**:
```bash
codesearch dfg <file> --format dot --export dfg.dot
codesearch dfg <file> --format json
```

**Use Cases**:
- Analyzing variable lifetimes
- Optimizing memory usage
- Parallelizing computations
- Identifying redundant computations

**Tests**: 4 unit tests

---

### 4. Call Graph
**Module**: `src/callgraph.rs` (420 LOC)

**What it represents**: Calling relationships between functions/methods. Nodes represent functions, edges represent function calls.

**Capabilities**:
- Build call graph across entire codebase
- Identify recursive functions
- Find dead (unused) functions
- Calculate call depth
- Find call chains between functions
- Track direct and indirect calls

**CLI Command**:
```bash
codesearch callgraph . -e rs,py,js --format dot
codesearch callgraph . --recursive-only
codesearch callgraph . --dead-only
```

**Use Cases**:
- Analyzing function dependencies
- Identifying unused or dead functions
- Understanding large codebases
- Detecting recursive or deeply nested calls

**Tests**: 4 unit tests

---

### 5. Dependency Graph
**Module**: `src/depgraph.rs` (380 LOC) - Enhanced

**What it represents**: Dependencies between modules, libraries, or variables.

**Capabilities**:
- Build module dependency graph
- Detect circular dependencies
- Identify root and leaf modules
- Calculate dependency depth
- Export to DOT format
- JSON export for programmatic analysis

**CLI Command**:
```bash
codesearch depgraph . -e rs --format dot
codesearch depgraph . --circular-only
```

**Use Cases**:
- Managing and minimizing dependencies
- Detecting circular dependencies
- Impact analysis (which modules affected by change)
- Architecture visualization

**Tests**: 4 unit tests

---

### 6. Program Dependency Graph (PDG)
**Module**: `src/pdg.rs` (420 LOC)

**What it represents**: Combines control dependencies (CFG) and data dependencies (DFG) into a single graph.

**Capabilities**:
- Merge CFG and DFG into unified view
- Backward program slicing (what affects a variable)
- Forward program slicing (what is affected by a variable)
- Find parallelization opportunities
- Taint analysis (security)
- Identify independent operations

**CLI Command**:
```bash
codesearch pdg <file> --format dot --export pdg.dot
codesearch pdg <file> --parallel
```

**Use Cases**:
- Advanced static code analysis
- Parallelization and optimization
- Security analysis (taint analysis)
- Program slicing (isolating code affecting specific output)

**Tests**: 4 unit tests

---

## Unified Graph Analysis

**Module**: `src/graphs.rs` (350 LOC)

Provides unified interface for all graph types with common analysis and export capabilities.

**CLI Command**:
```bash
codesearch graph-all <file> --export-dir ./graphs
```

Analyzes all applicable graph types for a file and exports results.

---

## Architecture

### Module Structure
```
src/
├── ast.rs          (420 LOC) - AST parsing with tree-sitter
├── cfg.rs          (350 LOC) - Control flow analysis
├── dfg.rs          (380 LOC) - Data flow analysis
├── callgraph.rs    (420 LOC) - Call graph construction
├── depgraph.rs     (380 LOC) - Dependency analysis
├── pdg.rs          (420 LOC) - Program dependency graph
└── graphs.rs       (350 LOC) - Unified interface
```

**Total**: ~2,720 lines of new code

### Data Flow
```
Source Code
    ↓
AST Parser (tree-sitter)
    ↓
    ├─→ CFG Builder → Control Flow Graph
    ├─→ DFG Builder → Data Flow Graph
    ├─→ Call Graph Builder → Call Graph
    ├─→ Dependency Builder → Dependency Graph
    └─→ PDG Builder (CFG + DFG) → Program Dependency Graph
```

---

## CLI Commands Summary

| Command | Purpose | Output Formats |
|---------|---------|----------------|
| `codesearch ast` | Parse AST | text, json |
| `codesearch cfg` | Control flow | text, json, dot |
| `codesearch dfg` | Data flow | text, json, dot |
| `codesearch callgraph` | Function calls | text, json, dot |
| `codesearch depgraph` | Module dependencies | text, json, dot |
| `codesearch pdg` | Program dependencies | text, json, dot |
| `codesearch graph-all` | All graphs | text, json |

---

## Export Formats

### DOT Format
All graph types (except AST) support DOT format export for visualization with Graphviz:

```bash
codesearch cfg file.rs --format dot --export cfg.dot
dot -Tpng cfg.dot -o cfg.png
```

### JSON Format
All graph types support JSON export for programmatic analysis:

```bash
codesearch dfg file.rs --format json > dfg.json
```

---

## Usage Examples

### 1. Find Unreachable Code
```bash
codesearch cfg src/main.rs
# Output shows unreachable blocks
```

### 2. Detect Unused Variables
```bash
codesearch dfg src/lib.rs
# Output shows unused variables
```

### 3. Find Dead Functions
```bash
codesearch callgraph . -e rs --dead-only
# Lists functions never called
```

### 4. Detect Circular Dependencies
```bash
codesearch depgraph . -e rs --circular-only
# Shows circular dependency cycles
```

### 5. Find Parallelization Opportunities
```bash
codesearch pdg src/compute.rs --parallel
# Shows independent operations that can run in parallel
```

### 6. Comprehensive Analysis
```bash
codesearch graph-all src/main.rs --export-dir ./analysis
# Generates all graph types and exports to directory
```

---

## Visualization

### Graphviz Integration
All graph types export to DOT format for visualization:

```bash
# Generate PNG visualization
codesearch cfg file.rs --format dot | dot -Tpng > cfg.png

# Generate SVG visualization
codesearch callgraph . --format dot | dot -Tsvg > callgraph.svg

# Interactive visualization
codesearch pdg file.rs --format dot | xdot -
```

### Color Coding

**CFG**:
- Green: Entry blocks
- Red: Exit/Return blocks
- Yellow: Branch blocks
- Blue: Loop blocks
- White: Normal blocks

**DFG**:
- Blue: Variables
- Green: Parameters
- Yellow: Constants
- Red: Operations
- Pink: Function calls

**Call Graph**:
- Green: Root functions (entry points)
- Red: Recursive functions
- Blue: Normal functions

**PDG**:
- Red edges: Control dependencies
- Blue edges: Data dependencies
- Purple edges: Both

---

## Performance Characteristics

### CFG
- **Complexity**: O(n) where n = lines of code
- **Memory**: O(b) where b = number of basic blocks
- **Best for**: Functions up to 1000 lines

### DFG
- **Complexity**: O(n*v) where n = lines, v = variables
- **Memory**: O(v + e) where e = data flow edges
- **Best for**: Functions with <100 variables

### Call Graph
- **Complexity**: O(f*c) where f = functions, c = calls per function
- **Memory**: O(f + c) where c = total calls
- **Best for**: Codebases up to 10,000 functions

### PDG
- **Complexity**: O(CFG + DFG)
- **Memory**: O(nodes + edges from both graphs)
- **Best for**: Individual function analysis

---

## Limitations

1. **Language Support**: Currently Rust, Python, JavaScript for AST-based analysis
2. **Interprocedural Analysis**: Call graph is intraprocedural (within same codebase)
3. **Dynamic Calls**: Cannot track function pointers or dynamic dispatch
4. **Aliasing**: Limited alias analysis in DFG
5. **Precision**: Regex-based fallback for unsupported languages

---

## Future Enhancements

1. **More Languages**: Add C/C++, Java, Go grammars
2. **Interprocedural PDG**: Cross-function dependency analysis
3. **Pointer Analysis**: Better alias tracking
4. **Interactive Visualization**: Web-based graph explorer
5. **Incremental Analysis**: Update graphs on code changes
6. **Machine Learning**: Pattern recognition in graphs

---

## Testing

**Total Tests**: 22 unit tests across 6 modules

Each module includes tests for:
- Graph construction
- Node/edge operations
- Analysis algorithms (unreachable code, cycles, etc.)
- Export functionality

Run tests:
```bash
cargo test cfg
cargo test dfg
cargo test callgraph
cargo test pdg
```

---

## Integration with Existing Features

### MCP Server
All graph types can be exposed via MCP for AI agent integration.

### Incremental Indexing
Graph results can be cached in the code index for faster repeated analysis.

### Export System
Graphs integrate with existing CSV/Markdown export functionality.

---

## Conclusion

Comprehensive graph analysis suite providing 6 different perspectives on code structure:

✅ **AST** - Syntax structure  
✅ **CFG** - Control flow  
✅ **DFG** - Data flow  
✅ **Call Graph** - Function relationships  
✅ **Dependency Graph** - Module dependencies  
✅ **PDG** - Combined control + data dependencies  

All with:
- Clean, maintainable code
- Comprehensive test coverage
- Multiple export formats
- CLI integration
- Visualization support
