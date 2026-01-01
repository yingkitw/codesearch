//! Language Definitions
//!
//! Comprehensive list of 48+ supported programming languages with their patterns.

use super::types::LanguageInfo;

/// All supported languages with their patterns
pub fn get_supported_languages() -> Vec<LanguageInfo> {
    vec![
        // Rust
        LanguageInfo {
            name: "Rust",
            extensions: &["rs"],
            function_patterns: &[r"fn\s+\w+", r"pub\s+fn\s+\w+", r"pub\s+async\s+fn\s+\w+", r"async\s+fn\s+\w+"],
            class_patterns: &[r"struct\s+\w+", r"enum\s+\w+", r"trait\s+\w+", r"impl\s+\w+"],
            comment_patterns: &[r"//", r"/\*", r"///", r"//!"],
            import_patterns: &[r"use\s+", r"mod\s+", r"extern\s+crate"],
        },
        // Python
        LanguageInfo {
            name: "Python",
            extensions: &["py", "pyw", "pyi"],
            function_patterns: &[r"def\s+\w+", r"async\s+def\s+\w+"],
            class_patterns: &[r"class\s+\w+"],
            comment_patterns: &[r"#", r"'''", r#"""""#],
            import_patterns: &[r"import\s+", r"from\s+\w+\s+import"],
        },
        // JavaScript
        LanguageInfo {
            name: "JavaScript",
            extensions: &["js", "mjs", "cjs", "jsx"],
            function_patterns: &[r"function\s+\w+", r"const\s+\w+\s*=\s*(async\s*)?\(", r"=>\s*\{", r"async\s+function\s+\w+"],
            class_patterns: &[r"class\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"import\s+", r"require\s*\(", r"export\s+"],
        },
        // TypeScript
        LanguageInfo {
            name: "TypeScript",
            extensions: &["ts", "tsx", "mts", "cts"],
            function_patterns: &[r"function\s+\w+", r"const\s+\w+\s*=\s*(async\s*)?\(", r"=>\s*\{", r"async\s+function\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"interface\s+\w+", r"type\s+\w+\s*="],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"import\s+", r"export\s+"],
        },
        // Go
        LanguageInfo {
            name: "Go",
            extensions: &["go"],
            function_patterns: &[r"func\s+\w+", r"func\s+\(\w+\s+\*?\w+\)\s+\w+"],
            class_patterns: &[r"type\s+\w+\s+struct", r"type\s+\w+\s+interface"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"import\s+", r"package\s+"],
        },
        // Java
        LanguageInfo {
            name: "Java",
            extensions: &["java"],
            function_patterns: &[r"(public|private|protected)?\s*(static\s+)?[\w<>\[\]]+\s+\w+\s*\("],
            class_patterns: &[r"class\s+\w+", r"interface\s+\w+", r"enum\s+\w+", r"@interface\s+\w+"],
            comment_patterns: &[r"//", r"/\*", r"/\*\*"],
            import_patterns: &[r"import\s+", r"package\s+"],
        },
        // Kotlin
        LanguageInfo {
            name: "Kotlin",
            extensions: &["kt", "kts"],
            function_patterns: &[r"fun\s+\w+", r"suspend\s+fun\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"object\s+\w+", r"interface\s+\w+", r"data\s+class\s+\w+", r"sealed\s+class\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"import\s+", r"package\s+"],
        },
        // Swift
        LanguageInfo {
            name: "Swift",
            extensions: &["swift"],
            function_patterns: &[r"func\s+\w+", r"@objc\s+func\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"struct\s+\w+", r"enum\s+\w+", r"protocol\s+\w+", r"extension\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"import\s+"],
        },
        // C
        LanguageInfo {
            name: "C",
            extensions: &["c", "h"],
            function_patterns: &[r"[\w\*]+\s+\w+\s*\([^)]*\)\s*\{"],
            class_patterns: &[r"struct\s+\w+", r"enum\s+\w+", r"typedef\s+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"#include\s+", r"#define\s+"],
        },
        // C++
        LanguageInfo {
            name: "C++",
            extensions: &["cpp", "cc", "cxx", "hpp", "hh", "hxx"],
            function_patterns: &[r"[\w\*:]+\s+\w+\s*\([^)]*\)\s*(const)?\s*\{", r"virtual\s+[\w\*]+\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"struct\s+\w+", r"enum\s+(class\s+)?\w+", r"namespace\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"#include\s+", r"using\s+namespace", r"using\s+\w+"],
        },
        // C#
        LanguageInfo {
            name: "C#",
            extensions: &["cs"],
            function_patterns: &[r"(public|private|protected|internal)?\s*(static\s+)?(async\s+)?[\w<>\[\]]+\s+\w+\s*\("],
            class_patterns: &[r"class\s+\w+", r"interface\s+\w+", r"struct\s+\w+", r"enum\s+\w+", r"record\s+\w+"],
            comment_patterns: &[r"//", r"/\*", r"///"],
            import_patterns: &[r"using\s+", r"namespace\s+"],
        },
        // PHP
        LanguageInfo {
            name: "PHP",
            extensions: &["php", "phtml", "php3", "php4", "php5", "phps"],
            function_patterns: &[r"function\s+\w+", r"public\s+function\s+\w+", r"private\s+function\s+\w+", r"protected\s+function\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"interface\s+\w+", r"trait\s+\w+"],
            comment_patterns: &[r"//", r"#", r"/\*"],
            import_patterns: &[r"use\s+", r"namespace\s+", r"require", r"include"],
        },
        // Ruby
        LanguageInfo {
            name: "Ruby",
            extensions: &["rb", "rake", "gemspec"],
            function_patterns: &[r"def\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"module\s+\w+"],
            comment_patterns: &[r"#", r"=begin"],
            import_patterns: &[r"require\s+", r"require_relative\s+", r"include\s+"],
        },
        // Scala
        LanguageInfo {
            name: "Scala",
            extensions: &["scala", "sc"],
            function_patterns: &[r"def\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"object\s+\w+", r"trait\s+\w+", r"case\s+class\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"import\s+", r"package\s+"],
        },
        // Perl
        LanguageInfo {
            name: "Perl",
            extensions: &["pl", "pm", "t"],
            function_patterns: &[r"sub\s+\w+"],
            class_patterns: &[r"package\s+\w+"],
            comment_patterns: &[r"#", r"=pod"],
            import_patterns: &[r"use\s+", r"require\s+"],
        },
        // Lua
        LanguageInfo {
            name: "Lua",
            extensions: &["lua"],
            function_patterns: &[r"function\s+\w+", r"local\s+function\s+\w+"],
            class_patterns: &[],
            comment_patterns: &[r"--", r"--\[\["],
            import_patterns: &[r"require\s*\("],
        },
        // Shell/Bash
        LanguageInfo {
            name: "Shell",
            extensions: &["sh", "bash", "zsh", "fish"],
            function_patterns: &[r"function\s+\w+", r"\w+\s*\(\)\s*\{"],
            class_patterns: &[],
            comment_patterns: &[r"#"],
            import_patterns: &[r"source\s+", r"\.\s+"],
        },
        // PowerShell
        LanguageInfo {
            name: "PowerShell",
            extensions: &["ps1", "psm1", "psd1"],
            function_patterns: &[r"function\s+\w+", r"Function\s+\w+"],
            class_patterns: &[r"class\s+\w+"],
            comment_patterns: &[r"#", r"<#"],
            import_patterns: &[r"Import-Module", r"using\s+module"],
        },
        // R
        LanguageInfo {
            name: "R",
            extensions: &["r", "R", "rmd", "Rmd"],
            function_patterns: &[r"\w+\s*<-\s*function\s*\(", r"\w+\s*=\s*function\s*\("],
            class_patterns: &[r"setClass\s*\(", r"setRefClass\s*\("],
            comment_patterns: &[r"#"],
            import_patterns: &[r"library\s*\(", r"require\s*\(", r"source\s*\("],
        },
        // Julia
        LanguageInfo {
            name: "Julia",
            extensions: &["jl"],
            function_patterns: &[r"function\s+\w+", r"\w+\(.*\)\s*="],
            class_patterns: &[r"struct\s+\w+", r"mutable\s+struct\s+\w+", r"abstract\s+type\s+\w+"],
            comment_patterns: &[r"#", r"#="],
            import_patterns: &[r"using\s+", r"import\s+", r"include\s*\("],
        },
        // Haskell
        LanguageInfo {
            name: "Haskell",
            extensions: &["hs", "lhs"],
            function_patterns: &[r"\w+\s*::\s*", r"\w+\s+\w+\s*="],
            class_patterns: &[r"data\s+\w+", r"newtype\s+\w+", r"class\s+\w+", r"instance\s+\w+"],
            comment_patterns: &[r"--", r"\{-"],
            import_patterns: &[r"import\s+", r"module\s+"],
        },
        // Elixir
        LanguageInfo {
            name: "Elixir",
            extensions: &["ex", "exs"],
            function_patterns: &[r"def\s+\w+", r"defp\s+\w+"],
            class_patterns: &[r"defmodule\s+\w+", r"defprotocol\s+\w+", r"defimpl\s+\w+"],
            comment_patterns: &[r"#"],
            import_patterns: &[r"import\s+", r"use\s+", r"require\s+", r"alias\s+"],
        },
        // Erlang
        LanguageInfo {
            name: "Erlang",
            extensions: &["erl", "hrl"],
            function_patterns: &[r"\w+\s*\([^)]*\)\s*->", r"-spec\s+\w+"],
            class_patterns: &[r"-module\s*\("],
            comment_patterns: &[r"%"],
            import_patterns: &[r"-import\s*\(", r"-include"],
        },
        // Clojure
        LanguageInfo {
            name: "Clojure",
            extensions: &["clj", "cljs", "cljc", "edn"],
            function_patterns: &[r"\(defn\s+\w+", r"\(defn-\s+\w+", r"\(fn\s+\w+"],
            class_patterns: &[r"\(defrecord\s+\w+", r"\(deftype\s+\w+", r"\(defprotocol\s+\w+"],
            comment_patterns: &[r";"],
            import_patterns: &[r"\(ns\s+", r"\(require\s+", r"\(use\s+", r"\(import\s+"],
        },
        // Dart
        LanguageInfo {
            name: "Dart",
            extensions: &["dart"],
            function_patterns: &[r"\w+\s+\w+\s*\([^)]*\)\s*(async)?\s*\{", r"void\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"mixin\s+\w+", r"extension\s+\w+"],
            comment_patterns: &[r"//", r"/\*", r"///"],
            import_patterns: &[r#"import\s+'"#, r#"export\s+'"#, r"part\s+"],
        },
        // Objective-C
        LanguageInfo {
            name: "Objective-C",
            extensions: &["m", "mm"],
            function_patterns: &[r"-\s*\([^)]+\)\s*\w+", r"\+\s*\([^)]+\)\s*\w+"],
            class_patterns: &[r"@interface\s+\w+", r"@implementation\s+\w+", r"@protocol\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"#import\s+", r"#include\s+", r"@import\s+"],
        },
        // Groovy
        LanguageInfo {
            name: "Groovy",
            extensions: &["groovy", "gvy", "gy", "gsh", "gradle"],
            function_patterns: &[r"def\s+\w+", r"void\s+\w+", r"(public|private|protected)\s+\w+\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"interface\s+\w+", r"trait\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"import\s+", r"package\s+"],
        },
        // SQL
        LanguageInfo {
            name: "SQL",
            extensions: &["sql", "ddl", "dml"],
            function_patterns: &[r"CREATE\s+(OR\s+REPLACE\s+)?FUNCTION\s+\w+", r"CREATE\s+(OR\s+REPLACE\s+)?PROCEDURE\s+\w+"],
            class_patterns: &[r"CREATE\s+TABLE\s+\w+", r"CREATE\s+VIEW\s+\w+", r"CREATE\s+INDEX\s+\w+"],
            comment_patterns: &[r"--", r"/\*"],
            import_patterns: &[],
        },
        // YAML/Config
        LanguageInfo {
            name: "YAML",
            extensions: &["yaml", "yml"],
            function_patterns: &[],
            class_patterns: &[],
            comment_patterns: &[r"#"],
            import_patterns: &[],
        },
        // TOML
        LanguageInfo {
            name: "TOML",
            extensions: &["toml"],
            function_patterns: &[],
            class_patterns: &[r"\[[\w\.]+\]"],
            comment_patterns: &[r"#"],
            import_patterns: &[],
        },
        // JSON
        LanguageInfo {
            name: "JSON",
            extensions: &["json", "jsonc"],
            function_patterns: &[],
            class_patterns: &[],
            comment_patterns: &[],
            import_patterns: &[],
        },
        // XML/HTML
        LanguageInfo {
            name: "XML/HTML",
            extensions: &["xml", "html", "htm", "xhtml", "svg"],
            function_patterns: &[],
            class_patterns: &[r"<\w+[^>]*>"],
            comment_patterns: &[r"<!--"],
            import_patterns: &[r"<link", r"<script", r"<import"],
        },
        // CSS/SCSS/LESS
        LanguageInfo {
            name: "CSS",
            extensions: &["css", "scss", "sass", "less", "styl"],
            function_patterns: &[r"@mixin\s+\w+", r"@function\s+\w+"],
            class_patterns: &[r"\.\w+\s*\{", r"#\w+\s*\{", r"@media\s+"],
            comment_patterns: &[r"/\*", r"//"],
            import_patterns: &[r"@import\s+", r"@use\s+"],
        },
        // Markdown
        LanguageInfo {
            name: "Markdown",
            extensions: &["md", "markdown", "mdown", "mkd"],
            function_patterns: &[],
            class_patterns: &[r"^#+\s+"],
            comment_patterns: &[],
            import_patterns: &[],
        },
        // Zig
        LanguageInfo {
            name: "Zig",
            extensions: &["zig"],
            function_patterns: &[r"fn\s+\w+", r"pub\s+fn\s+\w+"],
            class_patterns: &[r"const\s+\w+\s*=\s*struct", r"const\s+\w+\s*=\s*enum"],
            comment_patterns: &[r"//"],
            import_patterns: &[r"@import\s*\("],
        },
        // V
        LanguageInfo {
            name: "V",
            extensions: &["v"],
            function_patterns: &[r"fn\s+\w+", r"pub\s+fn\s+\w+"],
            class_patterns: &[r"struct\s+\w+", r"interface\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r"import\s+", r"module\s+"],
        },
        // Nim
        LanguageInfo {
            name: "Nim",
            extensions: &["nim", "nims"],
            function_patterns: &[r"proc\s+\w+", r"func\s+\w+", r"method\s+\w+", r"template\s+\w+", r"macro\s+\w+"],
            class_patterns: &[r"type\s+\w+", r"object\s+\w+"],
            comment_patterns: &[r"#", r"#\["],
            import_patterns: &[r"import\s+", r"from\s+\w+\s+import", r"include\s+"],
        },
        // Crystal
        LanguageInfo {
            name: "Crystal",
            extensions: &["cr"],
            function_patterns: &[r"def\s+\w+"],
            class_patterns: &[r"class\s+\w+", r"struct\s+\w+", r"module\s+\w+"],
            comment_patterns: &[r"#"],
            import_patterns: &[r"require\s+"],
        },
        // OCaml
        LanguageInfo {
            name: "OCaml",
            extensions: &["ml", "mli"],
            function_patterns: &[r"let\s+\w+", r"let\s+rec\s+\w+"],
            class_patterns: &[r"type\s+\w+", r"module\s+\w+", r"class\s+\w+"],
            comment_patterns: &[r"\(\*"],
            import_patterns: &[r"open\s+", r"include\s+"],
        },
        // F#
        LanguageInfo {
            name: "F#",
            extensions: &["fs", "fsi", "fsx"],
            function_patterns: &[r"let\s+\w+", r"let\s+rec\s+\w+", r"member\s+\w+"],
            class_patterns: &[r"type\s+\w+", r"module\s+\w+"],
            comment_patterns: &[r"//", r"\(\*"],
            import_patterns: &[r"open\s+", r"#load\s+"],
        },
        // Assembly
        LanguageInfo {
            name: "Assembly",
            extensions: &["asm", "s", "S"],
            function_patterns: &[r"\w+:\s*$", r"\.globl\s+\w+"],
            class_patterns: &[r"\.section\s+", r"\.text", r"\.data"],
            comment_patterns: &[r";", r"#", r"//"],
            import_patterns: &[r"\.include\s+", r"%include\s+"],
        },
        // Makefile
        LanguageInfo {
            name: "Makefile",
            extensions: &["makefile", "Makefile", "mk"],
            function_patterns: &[r"^\w+\s*:"],
            class_patterns: &[],
            comment_patterns: &[r"#"],
            import_patterns: &[r"include\s+"],
        },
        // Docker
        LanguageInfo {
            name: "Dockerfile",
            extensions: &["dockerfile", "Dockerfile"],
            function_patterns: &[],
            class_patterns: &[r"FROM\s+", r"WORKDIR\s+"],
            comment_patterns: &[r"#"],
            import_patterns: &[r"COPY\s+", r"ADD\s+"],
        },
        // Terraform/HCL
        LanguageInfo {
            name: "Terraform",
            extensions: &["tf", "tfvars", "hcl"],
            function_patterns: &[],
            class_patterns: &[r#"resource\s+""#, r#"module\s+""#, r#"variable\s+""#, r#"output\s+""#],
            comment_patterns: &[r"#", r"//", r"/\*"],
            import_patterns: &[r"source\s*="],
        },
        // Protocol Buffers
        LanguageInfo {
            name: "Protobuf",
            extensions: &["proto"],
            function_patterns: &[r"rpc\s+\w+"],
            class_patterns: &[r"message\s+\w+", r"service\s+\w+", r"enum\s+\w+"],
            comment_patterns: &[r"//", r"/\*"],
            import_patterns: &[r#"import\s+""#],
        },
        // GraphQL
        LanguageInfo {
            name: "GraphQL",
            extensions: &["graphql", "gql"],
            function_patterns: &[r"query\s+\w+", r"mutation\s+\w+", r"subscription\s+\w+"],
            class_patterns: &[r"type\s+\w+", r"interface\s+\w+", r"input\s+\w+", r"enum\s+\w+"],
            comment_patterns: &[r"#"],
            import_patterns: &[],
        },
        // Solidity (Smart Contracts)
        LanguageInfo {
            name: "Solidity",
            extensions: &["sol"],
            function_patterns: &[r"function\s+\w+"],
            class_patterns: &[r"contract\s+\w+", r"interface\s+\w+", r"library\s+\w+", r"struct\s+\w+"],
            comment_patterns: &[r"//", r"/\*", r"///"],
            import_patterns: &[r"import\s+"],
        },
        // WebAssembly Text
        LanguageInfo {
            name: "WebAssembly",
            extensions: &["wat", "wast"],
            function_patterns: &[r"\(func\s+\$\w+"],
            class_patterns: &[r"\(module", r"\(type\s+\$\w+"],
            comment_patterns: &[r";;"],
            import_patterns: &[r"\(import\s+"],
        },
    ]
}
