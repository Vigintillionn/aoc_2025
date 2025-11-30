# 🎄 Advent of Code - Rust Skeleton

A robust, monolithic Rust template for solving Advent of Code problems. It includes an automated CLI to manage daily solutions, inputs, and testing, along with built-in benchmarking.

## Features

- **Monolithic Binary:** All days compiled into one executable for fast incremental builds.
- **Automated Scaffolding:** Generate daily boilerplate and register modules automatically.
- **Benchmarking:** built-in timing for every solution.
- **Test Macros:** Reduce boilerplate for example cases and regression testing.

## Usage

This project uses a custom runner inside `main.rs` to handle commands.

### 1. Create a new day

Generates `src/days/dayXX.rs` and `inputs/dayXX.txt`, then registers the module in `mod.rs`.

```bash
# Create the next available day automatically (e.g., if day01 exists, creates day02)
cargo run -- create

# Create a specific day
cargo run -- create 5
```

### 2. Run a solution

Runs the solution for a specific day against the real input and prints the execution time.

```bash
cargo run -- run 1
```

### 3. Run tests

Wrapper around `cargo test` to easily run tests for a specific day.

```bash
# Test a specific day
cargo run -- test 1

# Run all tests
cargo run -- test
```

### 4. Reset project

**WARNING**: Deletes all `day*.rs` files in `src/days` and all input files. Resets mod.rs.

```bash
cargo run -- reset
```

## Daily Workflow

1. Generate: Run `cargo run -- create`.
2. Input: Paste your puzzle input into the newly created `inputs/dayXX.txt`.
3. Test: Open `src/days/dayXX.rs`. Copy the example data from the puzzle description into the `example!` macro.
4. Solve: Implement part1 and part2. Run `cargo run -- test XX` to verify.
5. Run: Run `cargo run -- run XX` to get your answer and see how fast your code is!
6. Secure: Once solved, uncomment the `input_test!` macro to ensure future refactors don't break this day.
