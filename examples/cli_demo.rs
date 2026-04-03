//! CLI Demo Example
//! Demonstrates how to use faker_rust as a command line tool
//!
//! Run with: cargo run --bin faker -- [COMMAND]
//!
//! Examples:
//!   cargo run --bin faker -- name
//!   cargo run --bin faker -- name --first
//!   cargo run --bin faker -- email
//!   cargo run --bin faker -- address --full
//!   cargo run --bin faker -- company
//!   cargo run --bin faker -- phone
//!   cargo run --bin faker -- --seed 12345 name
//!   cargo run --bin faker -- list

use std::process::Command;

fn main() {
    println!("=== Faker CLI Demo ===\n");

    // Run CLI commands and show output
    run_cli_command("name", &[]);
    run_cli_command("name", &["--first"]);
    run_cli_command("email", &[]);
    run_cli_command("address", &["--full"]);
    run_cli_command("company", &[]);
    run_cli_command("phone", &[]);
    run_cli_command("internet", &["--username"]);
    run_cli_command("games", &["--pokemon"]);
    run_cli_command("movies", &["--star-wars"]);
    run_cli_command("tv", &["--simpsons"]);

    println!("\n=== Seeded (Deterministic) Generation ===\n");
    println!("Running: faker --seed 12345 name");
    for i in 0..3 {
        let output = Command::new("cargo")
            .args(["run", "--quiet", "--bin", "faker", "--", "--seed", "12345", "name"])
            .output()
            .expect("Failed to execute command");
        let result = String::from_utf8_lossy(&output.stdout);
        println!("  Run {}: {}", i + 1, result.trim());
    }
    println!("  (All runs produce the same output with same seed)");

    println!("\n=== Multiple Values ===\n");
    println!("Running: faker -c 5 name");
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "faker", "--", "-c", "5", "name"])
        .output()
        .expect("Failed to execute command");
    let result = String::from_utf8_lossy(&output.stdout);
    println!("{}", result);

    println!("\n=== Available Commands ===\n");
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "faker", "--", "list"])
        .output()
        .expect("Failed to execute command");
    let result = String::from_utf8_lossy(&output.stdout);
    println!("{}", result);
}

fn run_cli_command(command: &str, args: &[&str]) {
    let mut cmd_args = vec!["run", "--quiet", "--bin", "faker", "--"];
    cmd_args.extend_from_slice(args);
    cmd_args.push(command);

    let output = Command::new("cargo")
        .args(&cmd_args)
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8_lossy(&output.stdout);
    let arg_str = if args.is_empty() {
        String::new()
    } else {
        format!(" {}", args.join(" "))
    };
    println!("$ faker{}{}", arg_str, command);
    println!("  → {}", result.trim());
    println!();
}
