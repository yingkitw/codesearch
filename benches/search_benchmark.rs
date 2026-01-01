//! Benchmarks for search operations
//!
//! Run with: cargo bench

use codesearch::search::search_code;
use codesearch::search::pure::*;
use codesearch::types::SearchOptions;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::fs;
use tempfile::tempdir;

fn benchmark_search_small(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    
    // Create small test files
    for i in 0..10 {
        fs::write(
            dir.path().join(format!("file{i}.rs")),
            "fn test() { println!(\"test\"); }",
        )
        .unwrap();
    }

    let options = SearchOptions::default();

    c.bench_function("search_small_10_files", |b| {
        b.iter(|| {
            search_code(black_box("test"), black_box(dir.path()), black_box(&options))
        })
    });
}

fn benchmark_search_medium(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    
    // Create medium test files
    for i in 0..100 {
        let content = format!(
            "fn test_function_{i}() {{\n    let x = {i};\n    println!(\"test: {{}}\", x);\n}}\n"
        );
        fs::write(dir.path().join(format!("file{i}.rs")), content).unwrap();
    }

    let options = SearchOptions::default();

    c.bench_function("search_medium_100_files", |b| {
        b.iter(|| {
            search_code(black_box("test"), black_box(dir.path()), black_box(&options))
        })
    });
}

fn benchmark_relevance_score(c: &mut Criterion) {
    c.bench_function("relevance_score_calculation", |b| {
        b.iter(|| {
            calculate_relevance_score_pure(
                black_box("fn test_function() {"),
                black_box("test"),
                black_box(10),
                black_box(Some("rs")),
                black_box(false),
                black_box(None),
            )
        })
    });
}

fn benchmark_fuzzy_match_quality(c: &mut Criterion) {
    c.bench_function("fuzzy_match_quality", |b| {
        b.iter(|| {
            fuzzy_match_quality(
                black_box(100),
                black_box(4),
                black_box(50),
            )
        })
    });
}

fn benchmark_search_with_options(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    
    for i in 0..50 {
        fs::write(
            dir.path().join(format!("file{i}.rs")),
            "fn test() { println!(\"test\"); }",
        )
        .unwrap();
    }

    let mut group = c.benchmark_group("search_with_options");
    
    for fuzzy in [false, true].iter() {
        group.bench_with_input(
            BenchmarkId::new("fuzzy", fuzzy),
            fuzzy,
            |b, &fuzzy| {
                let options = SearchOptions {
                    fuzzy,
                    ..Default::default()
                };
                b.iter(|| {
                    search_code(black_box("test"), black_box(dir.path()), black_box(&options))
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_pure_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("pure_functions");
    
    group.bench_function("should_include_line", |b| {
        b.iter(|| {
            should_include_line(
                black_box("test line content"),
                black_box(5),
                black_box(100),
                black_box(&["exclude"]),
            )
        })
    });
    
    group.bench_function("relevance_category", |b| {
        b.iter(|| {
            relevance_category(black_box(75.0))
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_search_small,
    benchmark_search_medium,
    benchmark_relevance_score,
    benchmark_fuzzy_match_quality,
    benchmark_search_with_options,
    benchmark_pure_functions,
);
criterion_main!(benches);
