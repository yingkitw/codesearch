# Design Metrics Implementation

## Overview

Comprehensive implementation of software design quality metrics for measuring coupling, cohesion, and architectural stability.

## Implemented Metrics

### 1. Afferent Coupling (Ca)

**Definition**: Number of classes or modules depending on a given module.

**Interpretation**:
- **High Ca (>5)**: Module is critical; many other modules depend on it
- **Impact**: Changes to this module affect many other modules
- **Use Case**: Identify core/critical modules that need extra care

**Example**:
```
Module A: Ca = 8
→ 8 other modules depend on Module A
→ Changes to A will impact 8 modules
→ A is a critical module
```

### 2. Efferent Coupling (Ce)

**Definition**: Number of classes or modules a given module depends on.

**Interpretation**:
- **High Ce (>5)**: Module has high reliance on external components
- **Impact**: Changes in dependencies affect this module
- **Use Case**: Identify modules with too many dependencies

**Example**:
```
Module B: Ce = 10
→ Module B depends on 10 other modules
→ Changes in any of those 10 modules may affect B
→ B is highly dependent
```

### 3. Instability (I)

**Definition**: Measures the balance between afferent and efferent coupling.

**Formula**: `I = Ce / (Ca + Ce)`

**Range**: 0.0 to 1.0

**Interpretation**:
- **I = 0.0**: Maximally stable (many dependents, no dependencies)
- **I = 1.0**: Maximally unstable (no dependents, many dependencies)
- **I < 0.3**: Stable module (good for core/infrastructure)
- **I > 0.7**: Unstable module (good for high-level/UI code)

**Example**:
```
Module C: Ca = 6, Ce = 2
I = 2 / (6 + 2) = 0.25 (Stable)

Module D: Ca = 1, Ce = 8
I = 8 / (1 + 8) = 0.89 (Unstable)
```

### 4. Abstractness (A)

**Definition**: Ratio of abstract classes/interfaces to total classes in a module.

**Formula**: `A = Abstract Classes / Total Classes`

**Range**: 0.0 to 1.0

**Interpretation**:
- **A = 0.0**: Completely concrete (no abstractions)
- **A = 1.0**: Completely abstract (all interfaces/traits)
- **High A**: More flexible, easier to extend

### 5. Distance from Main Sequence (D)

**Definition**: Measures how far a module is from the ideal balance of stability and abstractness.

**Formula**: `D = |A + I - 1|`

**Range**: 0.0 to 1.0

**Interpretation**:
- **D = 0.0**: On the main sequence (ideal balance)
- **D > 0.5**: Far from ideal (problematic)
- **Zone of Pain**: High stability (I=0), Low abstractness (A=0)
- **Zone of Uselessness**: Low stability (I=1), High abstractness (A=1)

### 6. Package Cohesion (LCOM)

**Definition**: Lack of Cohesion of Methods - measures how well classes in a package work together.

**Formula**: `LCOM = Non-sharing method pairs / Total method pairs`

**Range**: 0.0 to 1.0

**Interpretation**:
- **LCOM = 0.0**: Perfect cohesion (all methods share fields)
- **LCOM = 1.0**: No cohesion (no methods share fields)
- **LCOM < 0.5**: Good cohesion
- **LCOM > 0.7**: Poor cohesion (consider splitting class)

**Module Cohesion**: `Cohesion = 1 - Average(LCOM)`

---

## Implementation Details

### Module: `src/designmetrics.rs` (680 LOC)

**Data Structures**:
```rust
pub struct DesignMetrics {
    pub modules: HashMap<String, ModuleMetrics>,
    pub overall_stats: OverallStats,
}

pub struct ModuleMetrics {
    pub afferent_coupling: usize,      // Ca
    pub efferent_coupling: usize,      // Ce
    pub instability: f64,              // I
    pub abstractness: f64,             // A
    pub distance_from_main: f64,       // D
    pub cohesion: f64,                 // 1 - LCOM
    pub classes: Vec<ClassMetrics>,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
}

pub struct ClassMetrics {
    pub methods: Vec<String>,
    pub fields: Vec<String>,
    pub method_field_usage: HashMap<String, HashSet<String>>,
    pub lcom: f64,
}
```

**Analysis Process**:
1. **First Pass**: Scan all files, extract dependencies
2. **Calculate Ce**: Count dependencies for each module
3. **Second Pass**: Calculate Ca by finding reverse dependencies
4. **Calculate I**: Apply formula I = Ce / (Ca + Ce)
5. **Extract Classes**: Parse class structures
6. **Calculate LCOM**: Analyze method-field usage
7. **Calculate Cohesion**: Average LCOM across classes
8. **Overall Stats**: Aggregate metrics across all modules

---

## CLI Usage

### Basic Analysis
```bash
codesearch design-metrics .
```

### With Filters
```bash
# Specific extensions
codesearch design-metrics . -e rs,py,js

# Exclude directories
codesearch design-metrics . --exclude target,node_modules

# Detailed output
codesearch design-metrics . --detailed

# JSON output
codesearch design-metrics . --format json
```

---

## Output Examples

### Summary Output
```
Design Metrics Analysis
============================================================

Overall Statistics:
  Total modules: 25
  Average afferent coupling (Ca): 2.4
  Average efferent coupling (Ce): 3.1
  Average instability (I): 0.56
  Average cohesion: 0.78

⚠️  Highly Coupled Modules:
  - core_module (Ca=8, Ce=10)
  - utils (Ca=12, Ce=3)

⚠️  Unstable Modules (I > 0.7):
  - ui_component (I = 0.85)
  - temp_handler (I = 0.92)

⚠️  Low Cohesion Modules:
  - mixed_utils (cohesion = 0.35)

🔴 Critical Modules (Ca > 5):
  - core_module (Ca = 8)
  - utils (Ca = 12)

✅ Stable Modules (I < 0.3):
  - database (I = 0.15)
  - config (I = 0.22)
```

### Detailed Module Output
```
Module: core_module
  File: src/core/module.rs
  Afferent Coupling (Ca): 8 modules depend on this
  Efferent Coupling (Ce): 3 dependencies
  Instability (I): 0.27
  Abstractness (A): 0.60
  Distance from Main: 0.13
  Cohesion: 0.85
  Dependents: ui, api, service, worker, cache, logger, metrics, monitor
  Dependencies: config, database, utils
  Classes:
    - CoreService (LCOM: 0.15, methods: 12, fields: 8)
    - CoreHandler (LCOM: 0.20, methods: 8, fields: 5)
```

---

## Interpretation Guide

### Ideal Module Characteristics

**Stable Core Modules** (Infrastructure):
- Low Instability (I < 0.3)
- High Abstractness (A > 0.5)
- High Afferent Coupling (Ca > 5)
- Low Efferent Coupling (Ce < 3)
- High Cohesion (> 0.7)

**Flexible Application Modules** (UI/Business Logic):
- High Instability (I > 0.7)
- Low Abstractness (A < 0.5)
- Low Afferent Coupling (Ca < 3)
- Moderate Efferent Coupling (Ce 3-7)
- High Cohesion (> 0.7)

### Warning Signs

**🔴 Critical Issues**:
- Ca > 10: Too many dependents (refactor into smaller modules)
- Ce > 10: Too many dependencies (violates SRP)
- I > 0.9 with Ca > 5: Unstable but critical (dangerous)
- Cohesion < 0.3: Poor class design (split classes)
- D > 0.7: Far from main sequence (architectural problem)

**⚠️ Moderate Issues**:
- Ca > 5: Critical module (needs careful testing)
- Ce > 5: High coupling (consider dependency injection)
- I > 0.7: Unstable (acceptable for UI/high-level code)
- Cohesion < 0.5: Low cohesion (review class responsibilities)

**✅ Good Indicators**:
- 0.2 < I < 0.6: Balanced stability
- Cohesion > 0.7: Well-designed classes
- D < 0.3: Near main sequence
- Ca + Ce < 8: Reasonable coupling

---

## Use Cases

### 1. Refactoring Prioritization
```bash
codesearch design-metrics . --detailed
```
Focus on:
- Modules with high coupling (Ca + Ce > 15)
- Modules with low cohesion (< 0.5)
- Modules far from main sequence (D > 0.5)

### 2. Impact Analysis
```bash
codesearch design-metrics . | grep "Critical Modules"
```
Before changing a module, check its Ca:
- Ca > 5: High impact change (extensive testing needed)
- Ca < 2: Low impact change (safer to modify)

### 3. Architecture Validation
```bash
codesearch design-metrics . --format json > metrics.json
```
Validate architectural principles:
- Core modules should be stable (I < 0.3)
- UI modules should be unstable (I > 0.7)
- All modules should have good cohesion (> 0.6)

### 4. Code Review
Check new modules:
- Ensure reasonable coupling (Ca + Ce < 10)
- Verify good cohesion (> 0.6)
- Check stability matches layer (core=stable, UI=unstable)

---

## Integration with Other Metrics

### Combined Analysis
```bash
# Design + Complexity
codesearch design-metrics . --detailed
codesearch complexity . --threshold 10

# Design + Dependencies
codesearch design-metrics .
codesearch depgraph . --circular-only

# Design + Dead Code
codesearch design-metrics .
codesearch deadcode .
```

### Correlation Insights
- **High Ce + High Complexity**: Overly complex module
- **High Ca + Dead Code**: Critical but unused code
- **Low Cohesion + Duplicates**: Poor abstraction
- **High I + Circular Deps**: Architectural issues

---

## Language Support

**Full Support** (with class/method extraction):
- Rust (struct, enum, trait, impl)
- Python (class, def)
- JavaScript/TypeScript (class, function)
- Java/Kotlin (class, interface, method)
- Go (struct, func)

**Partial Support** (dependency analysis only):
- C/C++ (header dependencies)
- Other languages (import/use statements)

---

## Performance

**Analysis Speed**:
- Small projects (<100 files): < 1 second
- Medium projects (100-1000 files): 1-5 seconds
- Large projects (1000-10000 files): 5-30 seconds

**Memory Usage**:
- Proportional to number of modules and classes
- Typical: 10-50 MB for medium projects

---

## Limitations

1. **Dynamic Dependencies**: Cannot track runtime/reflection-based dependencies
2. **Indirect Coupling**: Only measures direct dependencies
3. **Language Features**: Limited support for advanced features (macros, generics)
4. **Heuristic-Based**: Uses pattern matching, not full semantic analysis
5. **Single Codebase**: Doesn't track external library dependencies

---

## Best Practices

### For Stable Modules (Core/Infrastructure)
- Keep I < 0.3 (stable)
- Maximize A (use traits/interfaces)
- Minimize Ce (few dependencies)
- Accept high Ca (many dependents is OK)

### For Unstable Modules (UI/Application)
- Allow I > 0.7 (unstable is OK)
- Keep A low (concrete implementations)
- Manage Ce (reasonable dependencies)
- Keep Ca low (few dependents)

### For All Modules
- Maintain cohesion > 0.6
- Keep Ca + Ce < 10
- Stay near main sequence (D < 0.3)
- Regular monitoring and refactoring

---

## Testing

**Unit Tests**: 6 tests covering:
- Module metrics creation
- Instability calculation
- LCOM calculation
- Critical module detection
- Zero coupling edge cases

Run tests:
```bash
cargo test designmetrics
```

---

## Future Enhancements

1. **Temporal Analysis**: Track metrics over time
2. **Threshold Configuration**: Customizable warning levels
3. **Visualization**: Generate coupling/cohesion graphs
4. **Recommendations**: Automated refactoring suggestions
5. **Team Metrics**: Per-developer/team coupling analysis
6. **CI Integration**: Fail builds on metric violations

---

## References

- **Martin, Robert C.** - "Clean Architecture" (Stability metrics)
- **Chidamber & Kemerer** - "A Metrics Suite for Object-Oriented Design" (LCOM)
- **ISO/IEC 25010** - Software Quality Model
- **Fowler, Martin** - "Refactoring" (Code smells related to coupling)

---

## Conclusion

Design metrics provide quantitative measures of software architecture quality:

✅ **Afferent Coupling (Ca)** - Measures module criticality  
✅ **Efferent Coupling (Ce)** - Measures dependency burden  
✅ **Instability (I)** - Measures flexibility vs stability  
✅ **Abstractness (A)** - Measures abstraction level  
✅ **Distance (D)** - Measures architectural health  
✅ **Cohesion** - Measures class design quality  

Use these metrics to:
- Identify refactoring priorities
- Assess architectural health
- Guide design decisions
- Monitor technical debt
- Improve code maintainability
