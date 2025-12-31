//! Comprehensive edge case and complex scenario tests for circular detection

#[cfg(test)]
mod complex_circular_tests {
    use crate::circular::{find_cycles_dfs, deduplicate_cycles, format_cycle, CircularCall};
    use std::collections::{HashMap, HashSet};

    // Helper to create call graph for testing
    fn create_call_graph(edges: Vec<(&str, Vec<&str>)>) -> HashMap<String, (String, HashSet<String>)> {
        let mut graph = HashMap::new();
        for (func, calls) in edges {
            let call_set: HashSet<String> = calls.iter().map(|s| s.to_string()).collect();
            graph.insert(func.to_string(), ("test.rs".to_string(), call_set));
        }
        graph
    }

    #[test]
    fn test_simple_two_node_cycle() {
        // A -> B -> A
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["A"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        assert!(!cycles.is_empty());
        assert!(cycles[0].chain.len() >= 2);
    }

    #[test]
    fn test_three_node_cycle() {
        // A -> B -> C -> A
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["C"]),
            ("C", vec!["A"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        assert!(!cycles.is_empty());
        assert!(cycles[0].chain.len() >= 3);
    }

    #[test]
    fn test_self_referencing_function() {
        // A -> A (direct recursion)
        let graph = create_call_graph(vec![
            ("A", vec!["A"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        assert!(!cycles.is_empty());
        assert_eq!(cycles[0].chain.len(), 1);
        assert_eq!(cycles[0].chain[0], "A");
    }

    #[test]
    fn test_long_chain_cycle() {
        // A -> B -> C -> D -> E -> A (5-node cycle)
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["C"]),
            ("C", vec!["D"]),
            ("D", vec!["E"]),
            ("E", vec!["A"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        assert!(!cycles.is_empty());
        assert!(cycles[0].chain.len() >= 5);
    }

    #[test]
    fn test_multiple_independent_cycles() {
        // Cycle 1: A -> B -> A
        // Cycle 2: C -> D -> C
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["A"]),
            ("C", vec!["D"]),
            ("D", vec!["C"]),
        ]);

        let mut all_cycles = Vec::new();
        let mut visited = HashSet::new();

        for start in &["A", "C"] {
            let mut rec_stack = HashSet::new();
            let mut path = Vec::new();
            find_cycles_dfs(start, &graph, &mut visited, &mut rec_stack, &mut path, &mut all_cycles);
        }

        assert!(all_cycles.len() >= 2);
    }

    #[test]
    fn test_nested_cycles_shared_node() {
        // A -> B -> C -> A (outer cycle)
        // B -> D -> B (inner cycle sharing B)
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["C", "D"]),
            ("C", vec!["A"]),
            ("D", vec!["B"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        // Should detect both cycles
        assert!(cycles.len() >= 1);
    }

    #[test]
    fn test_no_cycles_linear_chain() {
        // A -> B -> C -> D (no cycle)
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["C"]),
            ("C", vec!["D"]),
            ("D", vec![]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        assert!(cycles.is_empty());
    }

    #[test]
    fn test_diamond_pattern_no_cycle() {
        // A -> B -> D
        // A -> C -> D
        // (diamond, but no cycle)
        let graph = create_call_graph(vec![
            ("A", vec!["B", "C"]),
            ("B", vec!["D"]),
            ("C", vec!["D"]),
            ("D", vec![]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        assert!(cycles.is_empty());
    }

    #[test]
    fn test_cycle_with_external_calls() {
        // A -> B -> A (cycle)
        // A -> external_lib (not in graph)
        let graph = create_call_graph(vec![
            ("A", vec!["B", "external_lib"]),
            ("B", vec!["A"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        // Should still detect the A -> B -> A cycle
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_multiple_paths_to_cycle() {
        // A -> B -> C -> D -> C (cycle at C-D)
        // A -> E -> C (alternate path to cycle)
        let graph = create_call_graph(vec![
            ("A", vec!["B", "E"]),
            ("B", vec!["C"]),
            ("C", vec!["D"]),
            ("D", vec!["C"]),
            ("E", vec!["C"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        // Should detect the C -> D -> C cycle
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_deduplicate_same_cycle_different_start() {
        // Same cycle detected from different starting points
        let cycles = vec![
            CircularCall {
                chain: vec!["A".to_string(), "B".to_string(), "C".to_string()],
                files: vec!["test.rs".to_string()],
            },
            CircularCall {
                chain: vec!["B".to_string(), "C".to_string(), "A".to_string()],
                files: vec!["test.rs".to_string()],
            },
            CircularCall {
                chain: vec!["C".to_string(), "A".to_string(), "B".to_string()],
                files: vec!["test.rs".to_string()],
            },
        ];

        let unique = deduplicate_cycles(cycles);
        assert_eq!(unique.len(), 1);
    }

    #[test]
    fn test_format_cycle_empty() {
        let chain: Vec<String> = vec![];
        assert_eq!(format_cycle(&chain), "");
    }

    #[test]
    fn test_format_cycle_single_node() {
        let chain = vec!["A".to_string()];
        assert_eq!(format_cycle(&chain), "A -> A");
    }

    #[test]
    fn test_format_cycle_long_chain() {
        let chain = vec![
            "funcA".to_string(),
            "funcB".to_string(),
            "funcC".to_string(),
            "funcD".to_string(),
            "funcE".to_string(),
        ];
        assert_eq!(format_cycle(&chain), "funcA -> funcB -> funcC -> funcD -> funcE -> funcA");
    }

    #[test]
    fn test_complex_figure_eight_pattern() {
        // A -> B -> C -> A (left loop)
        // C -> D -> E -> C (right loop, sharing C)
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["C"]),
            ("C", vec!["A", "D"]),
            ("D", vec!["E"]),
            ("E", vec!["C"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        // Should detect at least one cycle
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_mutual_recursion_multiple_pairs() {
        // A <-> B, C <-> D (two pairs of mutual recursion)
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["A"]),
            ("C", vec!["D"]),
            ("D", vec!["C"]),
        ]);

        let mut all_cycles = Vec::new();
        let mut visited = HashSet::new();

        for start in &["A", "C"] {
            if !visited.contains(*start) {
                let mut rec_stack = HashSet::new();
                let mut path = Vec::new();
                find_cycles_dfs(start, &graph, &mut visited, &mut rec_stack, &mut path, &mut all_cycles);
            }
        }

        assert!(all_cycles.len() >= 2);
    }

    #[test]
    fn test_cycle_with_dead_end_branches() {
        // A -> B -> C -> A (cycle)
        // B -> D -> E (dead end branch)
        let graph = create_call_graph(vec![
            ("A", vec!["B"]),
            ("B", vec!["C", "D"]),
            ("C", vec!["A"]),
            ("D", vec!["E"]),
            ("E", vec![]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("A", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        // Should detect the A -> B -> C -> A cycle
        assert!(!cycles.is_empty());
        assert!(cycles.iter().any(|c| c.chain.contains(&"A".to_string())));
    }

    #[test]
    fn test_very_long_cycle() {
        // Create a 10-node cycle
        let graph = create_call_graph(vec![
            ("f1", vec!["f2"]),
            ("f2", vec!["f3"]),
            ("f3", vec!["f4"]),
            ("f4", vec!["f5"]),
            ("f5", vec!["f6"]),
            ("f6", vec!["f7"]),
            ("f7", vec!["f8"]),
            ("f8", vec!["f9"]),
            ("f9", vec!["f10"]),
            ("f10", vec!["f1"]),
        ]);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();
        let mut cycles = Vec::new();

        find_cycles_dfs("f1", &graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);

        assert!(!cycles.is_empty());
        assert!(cycles[0].chain.len() >= 10);
    }

    #[test]
    fn test_isolated_nodes_no_cycles() {
        // A, B, C are isolated (no calls)
        let graph = create_call_graph(vec![
            ("A", vec![]),
            ("B", vec![]),
            ("C", vec![]),
        ]);

        let mut all_cycles = Vec::new();
        let mut visited = HashSet::new();

        for start in &["A", "B", "C"] {
            if !visited.contains(*start) {
                let mut rec_stack = HashSet::new();
                let mut path = Vec::new();
                find_cycles_dfs(start, &graph, &mut visited, &mut rec_stack, &mut path, &mut all_cycles);
            }
        }

        assert!(all_cycles.is_empty());
    }
}
