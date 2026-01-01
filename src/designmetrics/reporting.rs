//! Design Metrics Reporting
//!
//! Functions for displaying design metrics analysis results.

use super::types::DesignMetrics;
use colored::*;

pub fn print_design_metrics(metrics: &DesignMetrics, detailed: bool) {
    println!("\n{}", "Design Metrics Analysis".cyan().bold());
    println!("{}", "=".repeat(60).cyan());

    println!("\n{}", "Overall Statistics:".green().bold());
    println!("  Total modules: {}", metrics.overall_stats.total_modules);
    println!("  Average afferent coupling (Ca): {:.2}", metrics.overall_stats.avg_afferent_coupling);
    println!("  Average efferent coupling (Ce): {:.2}", metrics.overall_stats.avg_efferent_coupling);
    println!("  Average instability (I): {:.2}", metrics.overall_stats.avg_instability);
    println!("  Average cohesion: {:.2}", metrics.overall_stats.avg_cohesion);

    if !metrics.overall_stats.highly_coupled_modules.is_empty() {
        println!("\n{}", "⚠️  Highly Coupled Modules:".yellow().bold());
        for module in &metrics.overall_stats.highly_coupled_modules {
            println!("  - {}", module);
        }
    }

    if !metrics.overall_stats.unstable_modules.is_empty() {
        println!("\n{}", "⚠️  Unstable Modules (I > 0.7):".yellow().bold());
        for module in &metrics.overall_stats.unstable_modules {
            if let Some(m) = metrics.modules.get(module) {
                println!("  - {} (I = {:.2})", module, m.instability);
            }
        }
    }

    if !metrics.overall_stats.low_cohesion_modules.is_empty() {
        println!("\n{}", "⚠️  Low Cohesion Modules:".yellow().bold());
        for module in &metrics.overall_stats.low_cohesion_modules {
            if let Some(m) = metrics.modules.get(module) {
                println!("  - {} (cohesion = {:.2})", module, m.cohesion);
            }
        }
    }

    let critical = metrics.get_critical_modules();
    if !critical.is_empty() {
        println!("\n{}", "🔴 Critical Modules (Ca > 5):".red().bold());
        for module in &critical {
            if let Some(m) = metrics.modules.get(module) {
                println!("  - {} (Ca = {})", module, m.afferent_coupling);
            }
        }
    }

    let stable = metrics.get_stable_modules();
    if !stable.is_empty() {
        println!("\n{}", "✅ Stable Modules (I < 0.3):".green().bold());
        for module in &stable {
            if let Some(m) = metrics.modules.get(module) {
                println!("  - {} (I = {:.2})", module, m.instability);
            }
        }
    }

    if detailed {
        println!("\n{}", "Detailed Module Metrics:".cyan().bold());
        println!("{}", "=".repeat(60).cyan());

        let mut modules: Vec<_> = metrics.modules.values().collect();
        modules.sort_by(|a, b| b.afferent_coupling.cmp(&a.afferent_coupling));

        for module in modules {
            println!("\n{}", format!("Module: {}", module.module_name).green().bold());
            println!("  File: {}", module.file_path);
            println!("  Afferent Coupling (Ca): {} modules depend on this", module.afferent_coupling);
            println!("  Efferent Coupling (Ce): {} dependencies", module.efferent_coupling);
            println!("  Instability (I): {:.2}", module.instability);
            println!("  Abstractness (A): {:.2}", module.abstractness);
            println!("  Distance from Main: {:.2}", module.distance_from_main);
            println!("  Cohesion: {:.2}", module.cohesion);

            if !module.dependents.is_empty() {
                println!("  Dependents: {}", module.dependents.join(", "));
            }

            if !module.dependencies.is_empty() {
                println!("  Dependencies: {}", module.dependencies.join(", "));
            }

            if !module.classes.is_empty() {
                println!("  Classes:");
                for class in &module.classes {
                    println!("    - {} (LCOM: {:.2}, methods: {}, fields: {})",
                        class.class_name, class.lcom, class.methods.len(), class.fields.len());
                }
            }
        }
    }
}
