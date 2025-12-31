//! Data types for dead code detection

use serde::Serialize;

/// Dead code detection result
#[derive(Debug, Clone, Serialize)]
pub struct DeadCodeItem {
    pub file: String,
    pub line_number: usize,
    pub item_type: String,
    pub name: String,
    pub reason: String,
}
