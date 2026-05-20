---
name: Benchmark and Verify Factrs3 Calculations
description: Use this skill when you need to benchmark the performance or verify the accuracy of the factorial and digit-counting algorithms in factrs3.
---

# Benchmark and Verify Factrs3

This skill provides a structured method for compiling, bench-testing, and verifying both the parallelized factorial calculations and the highly-optimized decimal digit counting logic.

## 📋 Verification & Benchmarking Workflow

Follow these steps to perform verification and benchmarking:

### 1. Compile in Release Mode
For accurate timing and performance measurements, always compile with the `--release` flag:
```bash
cargo build --release
```

### 2. Run Standard Factorial Benchmark
Run the compiled binary with a large, standard input (e.g., $180,000$) to check computational speed:
```bash
./target/release/factrs3 180000
```
*Expected Outcome:*
*   The calculation should run and display the results in milliseconds (typically under 100ms depending on CPU cores).
*   The digit count for $180,000!$ should be exactly **801,623**.

### 3. Verify Digit-Counting Correctness
To verify that the custom bit-boundary optimization in `dec_digits()` matches standard string conversion:
*   Ensure that edge cases around powers of 10 and exact bit boundaries are tested.
*   Run unit tests specifically targeted at the correctness of `dec_digits()` and `fact()`:
    ```bash
    cargo test
    ```

### 4. Code Correctness Checklist
- [ ] No conversion to `String` is used inside `dec_digits` except as a fallback comparison in extremely rare edge cases.
- [ ] Parallel iterator `into_par_iter()` is utilized for the core product operation.
- [ ] The CLI handles invalid command-line inputs gracefully, giving a non-zero exit code.
