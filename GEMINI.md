# Factrs3 🚀 - Project Context & Guidelines

Welcome to **factrs3**, a highly optimized, concurrent factorial calculator and large-number digit estimator written in Rust (Edition 2024).

---

## 🏗️ Project Architecture

This is a standard Rust application structured as both a library and a binary:

*   **`src/lib.rs`**: Core high-performance computation logic.
    *   `fact(n: usize) -> BigUint`: Calculates $N!$ in parallel using `rayon` to distribute product computation across available CPU cores.
    *   `dec_digits(s: &BigUint) -> usize`: A highly efficient algorithm to find the exact decimal digit count of a `BigUint` using logarithmic upper/lower bounds. It avoids heavy string allocation (`s.to_string().len()`), falling back to a comparison with $10^k$ only in marginal cases where bit length matches exactly.
*   **`src/main.rs`**: CLI entry point.
    *   Supports both interactive prompting and direct command-line arguments (e.g., `cargo run -- 180000`).
    *   Provides user-friendly formatted output with thousands separators.
    *   Profiles and reports the calculation time in milliseconds.

---

## 🛠️ Key Dependencies

*   **`num` (BigUint)**: Used for arbitrary precision arithmetic.
*   **`rayon`**: Used for effortless, safe data parallelism over iterators.

---

## 📋 Coding Guidelines & Standards

When modifying or expanding **factrs3**, follow these rules:

1.  **Maintain Rust 2024 Standards**: Use modern, idiomatic Rust. Run `cargo clippy` and `cargo fmt` to verify code quality.
2.  **Performance first**: The main goal of this tool is pure computational speed.
    *   **Rayon Iterators**: Ensure parallel processing (`into_par_iter()`) is leveraged for big math.
    *   **Avoid Allocations**: Do not convert `BigUint` to string unless absolutely necessary for printing/rendering. Always use `dec_digits` for counting digits.
3.  **Executable Binary**: The compiled executable in the project root (`/factrs3`) is ignored by `.gitignore`. Always build with `--release` for performance checks.
4.  **CLI UX**: Keep messages clean, informative, and professional.

---

## 🚀 Common Commands

*   **Format code**: `cargo fmt`
*   **Run lints**: `cargo clippy --all-targets`
*   **Run tests**: `cargo test`
*   **Build optimized binary**: `cargo build --release`
*   **Run benchmark execution**: `./target/release/factrs3 180000`
