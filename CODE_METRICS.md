# Comprehensive Code Metrics Implementation

## Overview

Complete implementation of code complexity, size, and maintainability metrics for comprehensive code quality analysis.

---

## 1. Code Complexity Metrics

### Cyclomatic Complexity

**Definition**: Counts the number of linearly independent paths through the code based on control flow.

**Formula**: `CC = E - N + 2P`
- E = edges in control flow graph
- N = nodes in control flow graph  
- P = connected components

**Simplified Calculation**: `CC = 1 + decision_points`

**Interpretation**:
- **1-10**: Simple, low risk
- **11-20**: Moderate complexity, moderate risk
- **21-50**: Complex, high risk
- **>50**: Very complex, very high risk

**Use Cases**:
- Identify functions needing refactoring
- Estimate testing effort
- Assess code maintainability

---

### Halstead Metrics

**Definition**: Measures code complexity based on operators and operands.

**Components**:

1. **n1**: Unique operators (e.g., +, -, *, /, =, ==, if, for)
2. **n2**: Unique operands (e.g., variables, constants)
3. **N1**: Total operators
4. **N2**: Total operands
5. **Vocabulary**: `n = n1 + n2`
6. **Length**: `N = N1 + N2`
7. **Volume**: `V = N * log2(n)` - Information content
8. **Difficulty**: `D = (n1/2) * (N2/n2)` - Mental effort
9. **Effort**: `E = D * V` - Total mental effort
10. **Time**: `T = E / 18` - Estimated time (seconds)
11. **Bugs**: `B = V / 3000` - Estimated delivered bugs

**Interpretation**:
- **Volume < 1000**: Simple code
- **Volume 1000-8000**: Moderate complexity
- **Volume > 8000**: Complex code
- **Bugs**: Estimated number of bugs in delivered code

**Use Cases**:
- Estimate development time
- Predict bug density
- Assess code understandability

---

### Essential Complexity (EC)

**Definition**: Measures complexity of program structure by identifying unstructured constructs.

**Calculation**:
- Base complexity: 1
- Add 1 for each `goto` statement
- Add 1 for each nesting level > 3

**Interpretation**:
- **EC = 1**: Structured code (ideal)
- **EC > 1**: Contains unstructured elements
- **EC > 5**: Highly unstructured (needs refactoring)

**Use Cases**:
- Identify code needing restructuring
- Assess code readability
- Guide refactoring priorities

---

### NPath Complexity

**Definition**: Measures the number of possible execution paths through a function.

**Formula**: Product of decision points (exponential growth)

**Approximation**: `NPath ≈ 2^decision_count`

**Interpretation**:
- **< 200**: Simple function
- **200-1000**: Moderate complexity
- **> 1000**: Overly complex function

**Use Cases**:
- Detect overly complex functions
- Estimate test case requirements
- Guide function decomposition

---

## 2. Code Size Metrics

### Lines of Code (LOC)

**Variants**:

1. **Total Lines**: All lines including blanks and comments
2. **SLOC (Source Lines of Code)**: Actual code lines (excluding comments and blanks)
3. **LLOC (Logical Lines of Code)**: Executable statements

**Interpretation**:
- **SLOC < 100**: Small file (easy to maintain)
- **SLOC 100-500**: Medium file
- **SLOC > 500**: Large file (consider splitting)

---

### Number of Classes, Methods, Functions

**Metrics**:
- Total classes/structs/enums
- Total methods/functions
- Average methods per class

**Interpretation**:
- **Methods per class < 10**: Good modularity
- **Methods per class > 20**: Consider splitting class

---

### Code Density

**Formula**: `Code Density = SLOC / Total Lines`

**Interpretation**:
- **< 0.5**: Low density (many comments/blanks)
- **0.5-0.8**: Balanced
- **> 0.8**: High density (may need more comments)

---

### Comment-to-Code Ratio

**Formula**: `Comment Ratio = Comment Lines / SLOC`

**Interpretation**:
- **< 0.1**: Under-documented
- **0.1-0.3**: Well-documented
- **> 0.3**: Possibly over-documented

---

## 3. Maintainability Metrics

### Maintainability Index (MI)

**Definition**: Composite metric combining cyclomatic complexity, LOC, and Halstead volume.

**Formula**: `MI = 171 - 5.2*ln(V) - 0.23*G - 16.2*ln(LOC)`

**Normalized**: 0-100 scale

**Interpretation**:
- **80-100**: Excellent maintainability
- **60-79**: Good maintainability
- **40-59**: Moderate maintainability
- **20-39**: Poor maintainability
- **0-19**: Very poor maintainability

**Use Cases**:
- Overall code quality assessment
- Prioritize refactoring efforts
- Track quality trends over time

---

### Code Churn

**Definition**: Measures how often code changes.

**Metrics**:
- Lines added
- Lines deleted
- Lines modified
- Churn rate

**Interpretation**:
- **High churn**: Unstable code, frequent changes
- **Low churn**: Stable code
- **Sudden spikes**: Potential issues or refactoring

**Use Cases**:
- Identify unstable modules
- Predict bug-prone areas
- Assess code stability

---

### Depth of Inheritance Tree (DIT)

**Definition**: Maximum depth of class inheritance hierarchy.

**Interpretation**:
- **DIT ≤ 3**: Good design
- **DIT 4-5**: Acceptable
- **DIT > 5**: Deep hierarchy (hard to understand)

**Use Cases**:
- Assess OOP design quality
- Identify over-engineered hierarchies
- Guide refactoring

---

### Coupling Between Objects (CBO)

**Definition**: Number of other classes a class is coupled to.

**Interpretation**:
- **CBO < 5**: Low coupling (good)
- **CBO 5-10**: Moderate coupling
- **CBO > 10**: High coupling (poor modularity)

**Use Cases**:
- Assess module independence
- Identify tightly coupled code
- Guide decoupling efforts

---

### Lack of Cohesion in Methods (LCOM)

**Definition**: Measures how closely related methods in a class are.

**Formula**: Ratio of method pairs that don't share fields

**Interpretation**:
- **LCOM < 0.3**: High cohesion (good)
- **LCOM 0.3-0.7**: Moderate cohesion
- **LCOM > 0.7**: Low cohesion (poor class design)

**Use Cases**:
- Identify classes needing splitting
- Assess class design quality
- Guide refactoring

---

## Implementation Details

### Module: `src/codemetrics.rs` (850 LOC)

**Data Structures**:
```rust
pub struct FileMetrics {
    pub complexity: ComplexityMetrics,
    pub size: SizeMetrics,
    pub maintainability: MaintainabilityMetrics,
}

pub struct ComplexityMetrics {
    pub cyclomatic_complexity: usize,
    pub halstead: HalsteadMetrics,
    pub essential_complexity: usize,
    pub npath_complexity: u64,
}

pub struct SizeMetrics {
    pub total_lines: usize,
    pub source_lines: usize,
    pub logical_lines: usize,
    pub comment_lines: usize,
    pub code_density: f64,
    pub comment_ratio: f64,
}

pub struct MaintainabilityMetrics {
    pub maintainability_index: f64,
    pub code_churn: CodeChurn,
    pub depth_of_inheritance: usize,
    pub coupling_between_objects: usize,
    pub lack_of_cohesion: f64,
}
```

---

## CLI Usage

### Basic Analysis
```bash
codesearch metrics .
```

### With Filters
```bash
# Specific extensions
codesearch metrics . -e rs,py,js

# Exclude directories
codesearch metrics . --exclude target,node_modules

# Detailed output
codesearch metrics . --detailed

# JSON output
codesearch metrics . --format json
```

---

## Output Examples

### Summary Output
```
Comprehensive Code Metrics Report
======================================================================

Project Summary:
  Total files: 45
  Total lines: 12,543
  Source lines (SLOC): 9,234
  Total functions: 234
  Total classes: 56

Complexity Metrics:
  Avg Cyclomatic Complexity: 5.2
  Avg Halstead Volume: 342.5
  Estimated Bugs (Halstead): 2.3

Maintainability:
  Avg Maintainability Index: 72.5/100
  Rating: Good
```

### Detailed File Output
```
File: src/main.rs

  Size Metrics:
    Total lines: 450
    SLOC: 320
    LLOC: 280
    Comment lines: 85
    Code density: 71.11%
    Comment ratio: 26.56%

  Complexity Metrics:
    Cyclomatic: 15
    Essential: 2
    NPath: 512

  Halstead Metrics:
    Volume: 1250.5
    Difficulty: 12.3
    Effort: 15381.2
    Time (seconds): 854.5
    Estimated bugs: 0.42

  Maintainability:
    MI: 68.5/100
    DIT: 2
    CBO: 8
    LCOM: 0.35
```

---

## Language Support

**Full Support**:
- Rust (fn, struct, impl, use)
- Python (def, class, import)
- JavaScript/TypeScript (function, class, import)
- Java/Kotlin (class, method, import)
- Go (func, struct, import)

**Partial Support**:
- C/C++ (functions, includes)
- Other languages (basic LOC counting)

---

## Metric Correlations

### Quality Indicators

**High Quality Code**:
- Low Cyclomatic Complexity (< 10)
- High Maintainability Index (> 70)
- Low LCOM (< 0.3)
- Balanced Comment Ratio (0.1-0.3)
- Low CBO (< 5)

**Low Quality Code**:
- High Cyclomatic Complexity (> 20)
- Low Maintainability Index (< 40)
- High LCOM (> 0.7)
- Very low or very high Comment Ratio
- High CBO (> 10)

### Refactoring Priorities

**Priority 1** (Critical):
- MI < 40
- Cyclomatic > 30
- NPath > 10,000
- LCOM > 0.8

**Priority 2** (High):
- MI 40-60
- Cyclomatic 20-30
- NPath 1,000-10,000
- LCOM 0.6-0.8

**Priority 3** (Medium):
- MI 60-70
- Cyclomatic 10-20
- NPath 200-1,000
- LCOM 0.4-0.6

---

## Use Cases

### 1. Code Review
```bash
codesearch metrics src/new_feature.rs --detailed
```
Check if new code meets quality standards.

### 2. Technical Debt Assessment
```bash
codesearch metrics . --format json > metrics.json
```
Track metrics over time to monitor technical debt.

### 3. Refactoring Prioritization
```bash
codesearch metrics . --detailed | grep "MI:"
```
Identify files with lowest maintainability.

### 4. Testing Effort Estimation
```bash
codesearch metrics . --detailed | grep "Cyclomatic:"
```
Estimate test cases needed based on complexity.

### 5. Bug Prediction
```bash
codesearch metrics . --detailed | grep "Estimated bugs:"
```
Predict bug-prone areas using Halstead metrics.

---

## Integration with Other Metrics

### Combined Analysis
```bash
# Metrics + Design
codesearch metrics .
codesearch design-metrics .

# Metrics + Complexity
codesearch metrics .
codesearch complexity . --threshold 10

# Metrics + Dead Code
codesearch metrics .
codesearch deadcode .
```

### Correlation Insights
- **High Complexity + Low MI**: Urgent refactoring needed
- **High CBO + High Churn**: Unstable architecture
- **Low Comment Ratio + High Complexity**: Needs documentation
- **High LCOM + Many Methods**: Class needs splitting

---

## Performance

**Analysis Speed**:
- Small projects (<100 files): < 2 seconds
- Medium projects (100-1000 files): 2-10 seconds
- Large projects (1000-10000 files): 10-60 seconds

**Memory Usage**:
- Proportional to file count
- Typical: 20-100 MB for medium projects

---

## Best Practices

### For New Code
- Target MI > 70
- Keep Cyclomatic < 10
- Maintain Comment Ratio 0.1-0.3
- Keep LCOM < 0.5
- Limit CBO < 5

### For Legacy Code
- Gradually improve MI
- Refactor functions with Cyclomatic > 20
- Add comments where ratio < 0.1
- Split classes with LCOM > 0.7

### For Teams
- Set quality gates in CI/CD
- Track metrics trends
- Review metrics in code reviews
- Celebrate improvements

---

## Testing

**Unit Tests**: 4 comprehensive tests covering:
- Halstead calculation
- Size metrics
- Cyclomatic complexity
- Maintainability index

Run tests:
```bash
cargo test codemetrics
```

---

## Limitations

1. **Heuristic-Based**: Uses pattern matching, not full semantic analysis
2. **Language-Specific**: Best results with fully supported languages
3. **Dynamic Features**: Cannot analyze runtime behavior
4. **Approximations**: Some metrics use simplified formulas
5. **Context-Free**: Doesn't consider business logic complexity

---

## Future Enhancements

1. **Temporal Analysis**: Track metrics over time
2. **Predictive Models**: ML-based bug prediction
3. **Custom Thresholds**: Configurable quality gates
4. **IDE Integration**: Real-time metrics in editor
5. **Team Metrics**: Per-developer/team analysis
6. **Visualization**: Interactive charts and graphs

---

## References

- **McCabe, T.J.** - "A Complexity Measure" (Cyclomatic Complexity)
- **Halstead, M.H.** - "Elements of Software Science" (Halstead Metrics)
- **Oman & Hagemeister** - "Metrics for Assessing a Software System's Maintainability" (MI)
- **Chidamber & Kemerer** - "A Metrics Suite for Object-Oriented Design" (CBO, LCOM, DIT)
- **ISO/IEC 25010** - Software Quality Model

---

## Conclusion

Comprehensive code metrics provide quantitative measures of code quality across three dimensions:

✅ **Complexity Metrics**:
- Cyclomatic Complexity
- Halstead Metrics (11 sub-metrics)
- Essential Complexity
- NPath Complexity

✅ **Size Metrics**:
- Lines of Code (LOC, SLOC, LLOC)
- Number of Classes/Methods/Functions
- Code Density
- Comment-to-Code Ratio

✅ **Maintainability Metrics**:
- Maintainability Index (MI)
- Code Churn
- Depth of Inheritance Tree (DIT)
- Coupling Between Objects (CBO)
- Lack of Cohesion in Methods (LCOM)

Use these metrics to:
- Assess code quality objectively
- Prioritize refactoring efforts
- Predict bug-prone areas
- Estimate testing effort
- Track technical debt
- Improve code maintainability
