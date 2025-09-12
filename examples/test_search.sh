#!/bin/bash

# Test script to demonstrate code search capabilities
# Make sure to run this from the project root directory

echo "🔍 Code Search Tool Demo"
echo "========================"
echo

echo "1. Finding all class definitions across languages:"
echo "--------------------------------------------------"
cargo run -- search "class" --extensions py,js,ts,java --line-numbers
echo

echo "2. Finding all function definitions in Rust:"
echo "--------------------------------------------"
cargo run -- search "fn\\s+\\w+" --extensions rs --line-numbers
echo

echo "3. Finding async functions in JavaScript/TypeScript:"
echo "----------------------------------------------------"
cargo run -- search "async" --extensions js,ts --ignore-case --line-numbers
echo

echo "4. Finding error handling patterns:"
echo "-----------------------------------"
cargo run -- search "Error|Exception" --extensions rs,py,js,ts,go,java --line-numbers
echo

echo "5. Finding all imports in Python:"
echo "---------------------------------"
cargo run -- search "^import|^from" --extensions py --line-numbers
echo

echo "6. Finding all public methods in Rust:"
echo "--------------------------------------"
cargo run -- search "pub\\s+fn" --extensions rs --line-numbers
echo

echo "7. Finding all test functions:"
echo "------------------------------"
cargo run -- search "test_|@test|func Test" --extensions py,js,ts,go --line-numbers
echo

echo "8. Listing all example files:"
echo "-----------------------------"
cargo run -- files --extensions rs,py,js,ts,go,java
echo

echo "9. JSON output example (first 3 results):"
echo "------------------------------------------"
cargo run -- search "def\\s+\\w+" --extensions py --format json | head -20
echo

echo "✅ Demo completed!"
