//! Minimal xtask runner for the qrc workspace.
//!
//! Usage: `cargo xtask <task>`
//!
//! Available tasks:
//!   ci      — Run the full CI pipeline (fmt check, clippy, tests)
//!   test    — Run tests
//!   clippy  — Run clippy lints

use std::{
    env,
    process::{self, Command},
};

fn main() {
    let task = env::args().nth(1).unwrap_or_default();
    let result = match task.as_str() {
        "ci" => ci(),
        "test" => run("cargo", &["test", "--all-features"]),
        "clippy" => run(
            "cargo",
            &[
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ),
        other => {
            eprintln!("Unknown task: {other}\n\nUsage: cargo xtask <ci|test|clippy>");
            process::exit(1);
        }
    };
    if let Err(e) = result {
        eprintln!("Task failed: {e}");
        process::exit(1);
    }
}

fn ci() -> Result<(), String> {
    run("cargo", &["fmt", "--all", "--", "--check"])?;
    run(
        "cargo",
        &[
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run("cargo", &["test", "--all-features"])?;
    Ok(())
}

fn run(cmd: &str, args: &[&str]) -> Result<(), String> {
    eprintln!("  → {cmd} {}", args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| format!("failed to run {cmd}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{cmd} exited with {status}"))
    }
}
