// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Shared display helpers for qrc examples.
//!
//! Provides animated spinner + checkmark output for a polished CLI feel.

#![allow(dead_code, clippy::pedantic)]

use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const SPINNER: &[&str] = &[
    "\u{2807}", "\u{280b}", "\u{2819}", "\u{2838}", "\u{2834}", "\u{2826}", "\u{2847}", "\u{280f}",
];
const CHECK: &str = "\u{2713}";
const CROSS: &str = "\u{2717}";

/// Print the example header.
pub fn header(title: &str) {
    println!();
    println!("  \x1b[1m{title}\x1b[0m");
    println!();
}

fn spin(label: &str, done: &Arc<AtomicBool>) {
    let mut i = 0;
    while !done.load(Ordering::Relaxed) {
        let frame = SPINNER[i % SPINNER.len()];
        print!("\r  \x1b[36m{frame}\x1b[0m \x1b[90m{label}\x1b[0m\x1b[K");
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(80));
        i += 1;
    }
}

/// Run a task with an animated spinner, then show a checkmark.
pub fn task<F, R>(label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();
    let label_owned = label.to_string();

    let handle = thread::spawn(move || spin(&label_owned, &done_clone));

    let result = f();
    done.store(true, Ordering::Relaxed);
    let _ = handle.join();
    print!("\r  \x1b[32m{CHECK}\x1b[0m {label}\x1b[K\n");
    let _ = io::stdout().flush();

    result
}

/// Run a task, then print additional detail lines beneath the checkmark.
pub fn task_with_output<F>(label: &str, f: F)
where
    F: FnOnce() -> Vec<String>,
{
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();
    let label_owned = label.to_string();

    let handle = thread::spawn(move || spin(&label_owned, &done_clone));

    let lines = f();
    done.store(true, Ordering::Relaxed);
    let _ = handle.join();
    print!("\r  \x1b[32m{CHECK}\x1b[0m {label}\x1b[K\n");
    for line in &lines {
        println!("    \x1b[90m{line}\x1b[0m");
    }
    let _ = io::stdout().flush();
}

/// Run a task that may fail. Shows checkmark on success, cross on failure.
pub fn task_result<F, T, E>(label: &str, f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
    E: std::fmt::Display,
{
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();
    let label_owned = label.to_string();

    let handle = thread::spawn(move || spin(&label_owned, &done_clone));

    let result = f();
    done.store(true, Ordering::Relaxed);
    let _ = handle.join();

    match &result {
        Ok(_) => print!("\r  \x1b[32m{CHECK}\x1b[0m {label}\x1b[K\n"),
        Err(e) => print!("\r  \x1b[31m{CROSS}\x1b[0m {label} \x1b[90m({e})\x1b[0m\x1b[K\n"),
    }
    let _ = io::stdout().flush();

    result
}

/// Show a summary line.
pub fn summary(count: usize) {
    println!();
    println!("  \x1b[1;32m{CHECK} {count} operations completed.\x1b[0m");
    println!();
}

/// Create a temporary directory for example output files, run the closure,
/// then clean up. Returns the closure's result.
pub fn with_temp_dir<F, R>(prefix: &str, f: F) -> R
where
    F: FnOnce(&std::path::Path) -> R,
{
    let dir = std::env::temp_dir().join(format!("qrc-example-{prefix}"));
    let _ = std::fs::create_dir_all(&dir);
    let result = f(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    result
}
