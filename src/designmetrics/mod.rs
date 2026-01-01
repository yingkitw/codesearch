//! Design Metrics Module
//!
//! Measures software design quality including coupling, cohesion, and instability.

pub mod types;
pub mod analysis;
pub mod extractors;
pub mod reporting;

pub use types::{DesignMetrics, ModuleMetrics, ClassMetrics, OverallStats};
pub use analysis::analyze_design_metrics;
pub use reporting::print_design_metrics;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_module_metrics_creation() {
        let metrics = ModuleMetrics::new("test".to_string(), "test.rs".to_string());
        assert_eq!(metrics.module_name, "test");
        assert_eq!(metrics.afferent_coupling, 0);
        assert_eq!(metrics.efferent_coupling, 0);
    }

    #[test]
    fn test_instability_calculation() {
        let mut metrics = ModuleMetrics::new("test".to_string(), "test.rs".to_string());
        metrics.afferent_coupling = 3;
        metrics.efferent_coupling = 2;
        metrics.calculate_instability();
        
        assert_eq!(metrics.instability, 0.4);
    }

    #[test]
    fn test_instability_zero_coupling() {
        let mut metrics = ModuleMetrics::new("test".to_string(), "test.rs".to_string());
        metrics.calculate_instability();
        
        assert_eq!(metrics.instability, 0.0);
    }

    #[test]
    fn test_class_metrics_lcom() {
        let mut class = ClassMetrics::new("TestClass".to_string());
        class.methods = vec!["method1".to_string(), "method2".to_string()];
        class.fields = vec!["field1".to_string(), "field2".to_string()];
        
        let mut usage1 = HashSet::new();
        usage1.insert("field1".to_string());
        class.method_field_usage.insert("method1".to_string(), usage1);
        
        let mut usage2 = HashSet::new();
        usage2.insert("field2".to_string());
        class.method_field_usage.insert("method2".to_string(), usage2);
        
        class.calculate_lcom();
        
        assert_eq!(class.lcom, 1.0);
    }

    #[test]
    fn test_design_metrics_creation() {
        let metrics = DesignMetrics::new();
        assert_eq!(metrics.modules.len(), 0);
        assert_eq!(metrics.overall_stats.total_modules, 0);
    }

    #[test]
    fn test_critical_modules() {
        let mut metrics = DesignMetrics::new();
        
        let mut module1 = ModuleMetrics::new("critical".to_string(), "critical.rs".to_string());
        module1.afferent_coupling = 10;
        metrics.add_module(module1);
        
        let mut module2 = ModuleMetrics::new("normal".to_string(), "normal.rs".to_string());
        module2.afferent_coupling = 2;
        metrics.add_module(module2);
        
        let critical = metrics.get_critical_modules();
        assert_eq!(critical.len(), 1);
        assert!(critical.contains(&"critical".to_string()));
    }
}
