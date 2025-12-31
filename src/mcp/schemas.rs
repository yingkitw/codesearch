//! JSON Schema implementations for MCP types

use crate::types::{ComplexityMetrics, DuplicateBlock, FileInfo, Match, SearchResult};
use crate::deadcode::DeadCodeItem;
use schemars::JsonSchema;

impl JsonSchema for SearchResult {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("SearchResult")
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::default()
    }
}

impl JsonSchema for FileInfo {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("FileInfo")
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::default()
    }
}

impl JsonSchema for Match {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Match")
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::default()
    }
}

impl JsonSchema for ComplexityMetrics {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ComplexityMetrics")
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::default()
    }
}

impl JsonSchema for DeadCodeItem {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("DeadCodeItem")
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::default()
    }
}

impl JsonSchema for DuplicateBlock {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("DuplicateBlock")
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::Schema::default()
    }
}
