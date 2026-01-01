# TODO

## ✅ Completed

- [x] Implement basic code search functionality
- [x] Add fuzzy search support
- [x] Add interactive mode
- [x] Add codebase analysis
- [x] Add refactoring suggestions
- [x] Implement MCP server support (rmcp 0.10 with 7 tools: search, list, analyze, complexity, duplicates, deadcode, circular)
- [x] Add comprehensive unit tests (80+ tests)
- [x] Add integration tests (26 tests)
- [x] Simplify CLI usage with defaults
- [x] Add semantic search enhancement
- [x] Add caching system for performance
- [x] Update README with comprehensive documentation
- [x] Create architecture documentation
- [x] Add progress indicators for long-running searches
- [x] Add export functionality (CSV, Markdown)
- [x] Add keyboard shortcuts in interactive mode
- [x] Add code complexity metrics (cyclomatic & cognitive complexity)
- [x] Add code duplication detection
- [x] Add dead code detection with enhanced capabilities:
  - Unused variables and constants detection
  - Unreachable code detection (after return/break/continue)
  - Empty function detection (supports Python, Rust, JS, etc.)
  - TODO/FIXME/HACK/XXX/BUG marker detection
  - Commented-out code detection
  - Unused import detection
- [x] Add comprehensive multi-language support (48 languages)
- [x] Modularize codebase into smaller maintainable modules (19 modules)
- [x] Refactor deadcode.rs into modular structure (4 sub-modules for better maintainability)
- [x] Extract CLI definitions from main.rs to cli.rs module (reduced main.rs from 1050 to 624 lines)
- [x] Modularize codemetrics.rs into 5 submodules (complexity, size, maintainability, helpers, mod)
- [x] Modularize designmetrics.rs into 5 submodules (types, analysis, extractors, reporting, mod)
- [x] Modularize language.rs into 4 submodules (types, definitions, utilities, mod)
- [x] Modularize search.rs into 5 submodules (core, fuzzy, semantic, utilities, mod)
- [x] Remove unsubstantiated performance claims from documentation
- [x] Ensure all key capabilities are exposed to MCP (7 tools total)
- [x] Verify code maintainability and testability standards
- [x] Implement all 6 graph analysis types:
  - Abstract Syntax Tree (AST)
  - Control Flow Graph (CFG)
  - Data Flow Graph (DFG)
  - Call Graph
  - Dependency Graph (enhanced)
  - Program Dependency Graph (PDG)
- [x] Add unified graph analysis interface
- [x] Add CLI commands for all graph types
- [x] Add DOT format export for visualization
- [x] Add 22 unit tests for graph modules
- [x] Implement design metrics module:
  - Afferent Coupling (Ca)
  - Efferent Coupling (Ce)
  - Instability (I)
  - Abstractness (A)
  - Distance from Main Sequence (D)
  - Package Cohesion (LCOM)
- [x] Add CLI command for design metrics analysis
- [x] Add 6 unit tests for design metrics
- [x] Implement comprehensive code metrics module:
  - Cyclomatic Complexity
  - Halstead Metrics (11 sub-metrics)
  - Essential Complexity
  - NPath Complexity
  - Lines of Code (LOC, SLOC, LLOC)
  - Code Density & Comment Ratio
  - Maintainability Index (MI)
  - Code Churn
  - Depth of Inheritance Tree (DIT)
  - Coupling Between Objects (CBO)
  - Lack of Cohesion in Methods (LCOM)
- [x] Add CLI command for comprehensive metrics
- [x] Add 4 unit tests for code metrics

## 🔄 In Progress

- None currently

## 📋 Planned

### Performance Improvements
- [x] Add incremental indexing for large codebases
- [x] Implement file watching for real-time updates
- [x] Optimize memory usage for very large files

### Features
- [x] Add AST-based code analysis (beyond regex)
- [x] Add dependency graph analysis
- [x] Add support for git history search
- [x] Add support for searching in remote repositories

### User Experience
- [ ] Add search result preview pane

### Testing
- [x] Add MCP server integration tests (23 tests)
- [ ] Add performance benchmarks
- [ ] Add fuzz testing for edge cases
- [ ] Add more complex integration test scenarios
- [ ] Add test coverage reporting

### Documentation
- [ ] Add API documentation
- [ ] Add more usage examples

## 🐛 Known Issues

- None currently

## 💡 Ideas for Future

- [ ] Machine learning-based code pattern recognition
- [ ] Collaborative search patterns sharing
- [ ] Code search as a service (web API)
- [ ] Integration with code review tools
- [ ] Support for searching in binary files (with limits)
- [ ] Add support for searching in database schemas
- [ ] Add support for searching in configuration files

