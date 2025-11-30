use std::path::Path;
use std::time::Instant;
use std::{
    fs,
    io::{self, Write},
};

use advent_of_code_2025::days::get_solver;
use clap::{Parser, Subcommand};
use colored::*;

const TEMPLATE: &str = r#"use crate::Solution;

pub fn solve(input: &str) -> Solution {
    Solution {
        part1: part1(input).to_string(),
        part2: part2(input).to_string(),
    }
}

fn part1(input: &str) -> i64 {
    0
}

fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::{example, input_test};

    example!(test_part1, part1, "test input", 0);
    example!(test_part2, part2, "test input", 0);

    // input_test!("$DAY", 0, 0);
}
"#;

const DAY_MOD: &str = r#"
// [IMPORT_MARKER]

pub fn get_solver(day: u8) -> Option<fn(&str) -> crate::Solution> {
    match day {
        // [MATCH_MARKER]
        _ => None,
    }
}
"#;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a specific day's solution
    Run {
        /// The day to run
        day: u8,
    },
    /// Test one day's solution, or all days
    Test {
        /// Optional day to test (defaults to all)
        day: Option<u8>,
    },
    /// Creates template files for a new day
    Create {
        /// Optional day number, creates next day if omitted
        day: Option<u8>,
    },
    /// Remove all input and day files
    Reset,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { day } => handle_run(day),
        Commands::Test { day } => handle_test(day),
        Commands::Create { day } => handle_create(day),
        Commands::Reset => handle_reset(),
    }
}

fn handle_run(day: u8) {
    println!(
        "{}",
        format!("---- Advent of Code Day {} ----", day)
            .bold()
            .bright_blue()
    );

    let input_path = format!("inputs/day{:02}.txt", day);
    let input = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!(
                "{} Could not read input file: {}",
                "Error:".red(),
                input_path
            );
            return;
        }
    };

    let solver = match get_solver(day) {
        Some(s) => s,
        None => {
            eprintln!("{} Day {} is not implemented yet.", "Error:".red(), day);
            return;
        }
    };

    let start = Instant::now();
    let solution = solver(&input);
    let duration = start.elapsed();

    println!("{} {}", "Part 1:".green(), solution.part1);
    println!("{} {}", "Part 2:".green(), solution.part2);

    println!(
        "\n{} {}",
        "Time:".yellow(),
        format!("{:.2?}", duration).italic()
    );
}

fn handle_test(day: Option<u8>) {
    let mut args = vec!["test"];

    let day_filter;

    if let Some(day) = day {
        println!("{}", format!("Running tests for Day {:02}...", day).cyan());
        day_filter = format!("day{:02}", day);
        args.push(&day_filter);
    } else {
        println!("{}", "Running all tests...".cyan());
    }

    let status = std::process::Command::new("cargo")
        .args(&args)
        .status()
        .expect("Failed to execute cargo test");

    if !status.success() {
        std::process::exit(1);
    }
}

fn handle_reset() {
    println!(
        "{} This will DELETE all day source files and input files.",
        "WARNING:".red().bold()
    );
    print!("Are you sure you want to continue? (y/N): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input.trim().to_lowercase() != "y" {
        println!("Reset aborted.");
        return;
    }

    // Delete source files (src/days/day*.rs)
    if let Ok(dir) = fs::read_dir("src/days") {
        for entry in dir.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("day") && name.ends_with(".rs") {
                    let _ = fs::remove_file(&path);
                    println!("Deleted {}", name);
                }
            }
        }
    }

    // Delete input files (inputs/*.txt)
    if let Ok(dir) = fs::read_dir("inputs") {
        for entry in dir.flatten() {
            let path = entry.path();
            if path.is_file() {
                let _ = fs::remove_file(&path);
                println!(
                    "Deleted inputs/{}",
                    path.file_name().unwrap().to_string_lossy()
                );
            }
        }
    }

    fs::write("src/days/mod.rs", DAY_MOD.trim_start()).expect("Failed to reset mod.rs");
    println!("{} src/days/mod.rs reset", "Success:".green());

    // Touch cargo.toml to force rust_analyzer to reload
    if let Ok(content) = fs::read_to_string("Cargo.toml") {
        let _ = fs::write("Cargo.toml", content);
    }
}

fn handle_create(day: Option<u8>) {
    let day = day.unwrap_or_else(|| {
        let dir = fs::read_dir("src/days").unwrap();
        let max_day = dir
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().into_string().ok()?;
                if name.starts_with("day") && name.ends_with(".rs") {
                    name[3..5].parse::<u8>().ok()
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);
        max_day + 1
    });

    let day_str = format!("{:02}", day);
    let src_path = format!("src/days/day{}.rs", day_str);
    let input_path = format!("inputs/day{}.txt", day_str);

    if Path::new(&src_path).exists() {
        eprintln!("{} File {} already exists!", "Error:".red(), src_path);
        return;
    }

    fs::write(&src_path, TEMPLATE.replace("$DAY", &day_str)).expect("Failed to write source file");
    println!("{} Created {}", "Success:".green(), src_path);

    if !Path::new(&input_path).exists() {
        fs::File::create(&input_path).expect("Failed to create input file");
        println!("{} Created {}", "Success:".green(), input_path);
    }

    update_get_solver(day, &day_str);

    // Touch cargo.toml to force rust_analyzer to reload
    if let Ok(content) = fs::read_to_string("Cargo.toml") {
        let _ = fs::write("Cargo.toml", content);
    }
}

fn update_get_solver(day: u8, day_str: &str) {
    let mod_path = "src/days/mod.rs";
    let content = fs::read_to_string(mod_path).expect("Failed to read mod.rs");

    if content.contains(&format!("mod day{};", day_str)) {
        return;
    }

    let new_mod_line = format!("pub mod day{};\n// [IMPORT_MARKER]", day_str);
    let new_match_line = format!(
        "{} => Some(day{}::solve),\n        // [MATCH_MARKER]",
        day, day_str
    );

    let new_content = content
        .replace("// [IMPORT_MARKER]", &new_mod_line)
        .replace("// [MATCH_MARKER]", &new_match_line);

    fs::write(mod_path, new_content).expect("Failed to update mod.rs");
    println!("{} Registered day {} in mod.rs", "Success:".green(), day);
}
