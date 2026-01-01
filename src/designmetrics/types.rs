//! Design Metrics Types
//!
//! Data structures for design quality metrics.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignMetrics {
    pub modules: HashMap<String, ModuleMetrics>,
    pub overall_stats: OverallStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetrics {
    pub module_name: String,
    pub file_path: String,
    pub afferent_coupling: usize,
    pub efferent_coupling: usize,
    pub instability: f64,
    pub abstractness: f64,
    pub distance_from_main: f64,
    pub cohesion: f64,
    pub classes: Vec<ClassMetrics>,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMetrics {
    pub class_name: String,
    pub methods: Vec<String>,
    pub fields: Vec<String>,
    pub method_field_usage: HashMap<String, HashSet<String>>,
    pub lcom: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallStats {
    pub total_modules: usize,
    pub avg_afferent_coupling: f64,
    pub avg_efferent_coupling: f64,
    pub avg_instability: f64,
    pub avg_cohesion: f64,
    pub highly_coupled_modules: Vec<String>,
    pub unstable_modules: Vec<String>,
    pub low_cohesion_modules: Vec<String>,
}

impl DesignMetrics {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            overall_stats: OverallStats {
                total_modules: 0,
                avg_afferent_coupling: 0.0,
                avg_efferent_coupling: 0.0,
                avg_instability: 0.0,
                avg_cohesion: 0.0,
                highly_coupled_modules: Vec::new(),
                unstable_modules: Vec::new(),
                low_cohesion_modules: Vec::new(),
            },
        }
    }

    pub fn add_module(&mut self, metrics: ModuleMetrics) {
        self.modules.insert(metrics.module_name.clone(), metrics);
    }

    pub fn calculate_overall_stats(&mut self) {
        let module_count = self.modules.len();
        if module_count == 0 {
            return;
        }

        let total_ca: usize = self.modules.values().map(|m| m.afferent_coupling).sum();
        let total_ce: usize = self.modules.values().map(|m| m.efferent_coupling).sum();
        let total_instability: f64 = self.modules.values().map(|m| m.instability).sum();
        let total_cohesion: f64 = self.modules.values().map(|m| m.cohesion).sum();

        self.overall_stats.total_modules = module_count;
        self.overall_stats.avg_afferent_coupling = total_ca as f64 / module_count as f64;
        self.overall_stats.avg_efferent_coupling = total_ce as f64 / module_count as f64;
        self.overall_stats.avg_instability = total_instability / module_count as f64;
        self.overall_stats.avg_cohesion = total_cohesion / module_count as f64;

        let coupling_threshold = self.overall_stats.avg_afferent_coupling + self.overall_stats.avg_efferent_coupling;
        self.overall_stats.highly_coupled_modules = self.modules
            .iter()
            .filter(|(_, m)| (m.afferent_coupling + m.efferent_coupling) as f64 > coupling_threshold * 1.5)
            .map(|(name, _)| name.clone())
            .collect();

        self.overall_stats.unstable_modules = self.modules
            .iter()
            .filter(|(_, m)| m.instability > 0.7)
            .map(|(name, _)| name.clone())
            .collect();

        self.overall_stats.low_cohesion_modules = self.modules
            .iter()
            .filter(|(_, m)| m.cohesion < 0.5)
            .map(|(name, _)| name.clone())
            .collect();
    }

    pub fn get_critical_modules(&self) -> Vec<String> {
        self.modules
            .iter()
            .filter(|(_, m)| m.afferent_coupling > 5)
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub fn get_stable_modules(&self) -> Vec<String> {
        self.modules
            .iter()
            .filter(|(_, m)| m.instability < 0.3)
            .map(|(name, _)| name.clone())
            .collect()
    }
}

impl ModuleMetrics {
    pub fn new(module_name: String, file_path: String) -> Self {
        Self {
            module_name,
            file_path,
            afferent_coupling: 0,
            efferent_coupling: 0,
            instability: 0.0,
            abstractness: 0.0,
            distance_from_main: 0.0,
            cohesion: 1.0,
            classes: Vec::new(),
            dependencies: Vec::new(),
            dependents: Vec::new(),
        }
    }

    pub fn calculate_instability(&mut self) {
        let total = self.afferent_coupling + self.efferent_coupling;
        if total > 0 {
            self.instability = self.efferent_coupling as f64 / total as f64;
        } else {
            self.instability = 0.0;
        }
    }

    pub fn calculate_distance_from_main(&mut self) {
        self.distance_from_main = (self.abstractness + self.instability - 1.0).abs();
    }

    pub fn calculate_cohesion(&mut self) {
        if self.classes.is_empty() {
            self.cohesion = 1.0;
            return;
        }

        let total_lcom: f64 = self.classes.iter().map(|c| c.lcom).sum();
        self.cohesion = 1.0 - (total_lcom / self.classes.len() as f64);
        self.cohesion = self.cohesion.max(0.0).min(1.0);
    }
}

impl ClassMetrics {
    pub fn new(class_name: String) -> Self {
        Self {
            class_name,
            methods: Vec::new(),
            fields: Vec::new(),
            method_field_usage: HashMap::new(),
            lcom: 0.0,
        }
    }

    pub fn calculate_lcom(&mut self) {
        if self.methods.is_empty() || self.fields.is_empty() {
            self.lcom = 0.0;
            return;
        }

        let m = self.methods.len();

        let mut non_sharing_pairs = 0;

        for i in 0..m {
            for j in (i + 1)..m {
                let method1 = &self.methods[i];
                let method2 = &self.methods[j];

                let fields1 = self.method_field_usage.get(method1).cloned().unwrap_or_default();
                let fields2 = self.method_field_usage.get(method2).cloned().unwrap_or_default();

                if fields1.intersection(&fields2).count() == 0 {
                    non_sharing_pairs += 1;
                }
            }
        }

        let total_pairs = (m * (m - 1)) / 2;
        if total_pairs > 0 {
            self.lcom = non_sharing_pairs as f64 / total_pairs as f64;
        } else {
            self.lcom = 0.0;
        }
    }
}
