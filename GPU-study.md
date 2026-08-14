# GPU Acceleration Study for factrs3

> **Date:** 2026-06-09
> **Target GPU:** AMD Radeon RX 7600
> **Verdict:** Not recommended. CPU-side alternatives yield far better ROI.

---

## 1. Current Implementation

factrs3 computes `n!` using Rayon parallel reduction over `num::BigUint`:

```rust
// src/lib.rs
pub fn fact(n: usize) -> BigUint {
    (1..=n).into_par_iter().map(BigUint::from).product()
}
```

**Benchmark (n = 180,000):**
- Result: 867,780 decimal digits (~2.88 million bits, ~45,000 64-bit limbs)
- Computation time: **~82.6 ms**
- Digit counting (`dec_digits`): **~43.4 ms** (O(1) analytical method)

The `product()` reduction builds a binary multiplication tree. Late-stage multiplications operate on operands with tens of thousands of limbs, and carry propagation is inherently sequential per multiplication.

---

## 2. Target GPU: AMD Radeon RX 7600

| Specification | Value |
|---|---|
| Architecture | RDNA 3 — Navi 33 (**gfx1102**) |
| Process | 6 nm (TSMC) |
| Compute Units | 32 (2048 stream processors) |
| FP32 (theoretical) | ~21.75 TFLOPS |
| Memory | 8 GB GDDR6, 128-bit bus |
| Memory bandwidth | 288 GB/s |
| Boost clock | ~2655 MHz |
| TDP | 165 W |
| Launch | May 2023 |
| OpenCL | 2.2 (native) |
| DirectX | 12 Ultimate (12_2) |

**Original query mentioned RX 7400.** Comparison:

| Spec | RX 7400 | RX 7600 | Delta |
|---|---|---|---|
| Compute Units | 28 | 32 | +14% |
| FP32 TFLOPS | ~7.88 | ~21.75 | **+176%** |
| Bandwidth | ~173 GB/s | 288 GB/s | +66% |
| TDP | 43-55W | 165W | +200% |
| Launch | Aug 2025 | May 2023 | — |

The RX 7600 is substantially more powerful, but the algorithmic bottlenecks are independent of raw FLOPs.

---

## 3. GPU Compute Stack Availability for this GPU

### 3.1 OpenCL — ✅ Viable

- **Crate:** `ocl` (~750★, maintained), `opencl3`
- **Status:** Fully functional. OpenCL 2.2 is natively supported by the RX 7600 drivers (both AMDGPU-Pro and RADV/Mesa).
- **Trade-off:** Kernels must be written in OpenCL C99, not Rust. Cross-boundary type verification is manual.
- **This is the only production-ready path today.**

### 3.2 HIP / ROCm — ⚠️ Not Officially Supported

- **Crate:** `cubecl` (feature `"hip"`), `rocm-rs`
- **Status:** gfx1102 (Navi 33) is **not in the official ROCm 7.2 supported GPU list.**
  - GitHub issue [ROCm/librocdxg#32](https://github.com/ROCm/librocdxg/issues/32) requesting gfx1102 support — unresolved.
  - GitHub issue [ROCm/ROCm#5555](https://github.com/ROCm/ROCm/issues/5555) — user asking about RX 7600 on ROCm 5.7, not officially supported.
  - Framework Community (Feb 2025): confirmed gfx1102 not officially supported.
  - Windows HIP SDK 7.1.1: lists W7900 (gfx1100) but not gfx1102.
- **Workaround:** `HSA_OVERRIDE_GFX_VERSION=gfx1100` may force enablement, but stability is not guaranteed.
- **CubeCL HIP backend** requires ROCm installed on Linux. Without official gfx1102 support, this is fragile.

### 3.3 Vulkan Compute (SPIR-V) — ✅ Functional, Limited

- **Crates:** `wgpu`, `vulkano`, `ash`
- **Status:** Works via RADV/AMDVLK drivers.
- **Severe limitation:** No native `u64`/`i64` support in SPIR-V or WGSL for consumer GPUs. All 64-bit arithmetic must be emulated with `u32` pairs, increasing instruction count 3-4× and register pressure.
- **Rust GPU (`rust-gpu`)** targets SPIR-V and has the same limitation (issue [#307](https://github.com/Rust-GPU/rust-gpu/issues/307)).

### 3.4 Other Paths

| Path | Crate/Project | Status |
|---|---|---|
| CUDA | `cudarc` | NVIDIA only |
| Metal | `cubecl` (Metal backend) | Apple only |
| WebGPU | `wgpu` | Browser/JS context |

---

## 4. Algorithmic Analysis: Why GPU is a Poor Fit

### 4.1 The Core Operation

Factorial computation reduces to a tree of `BigUint × BigUint` multiplications. For n = 180,000:

```
Final result: ~2.88M bits = ~45,000 × 64-bit limbs
Total multiplications: ~180,000 in a binary tree of depth log₂(n) ≈ 18
```

Late-stage multiplications involve operands with **tens of thousands of limbs**.

### 4.2 Obstacle A — Carry Propagation is Inherently Sequential

Multiplying two N-limb numbers (schoolbook: O(N²), Karatsuba: O(N^1.58)) produces 2N partial-product limbs. These must be normalized via carry propagation — a sequential scan through all limbs.

On a GPU SIMT architecture, threads within a warp (32 on AMD RDNA 3) must follow the same instruction stream. Carry chains cause **warp divergence**: threads serialize, wasting 31/32 of compute capacity per divergent step.

**Mitigation** exists (carry-save delayed normalization), but requires:
- A separate normalization kernel pass
- Careful tiling of the convolution
- Months of development by an experienced GPGPU engineer

The 2022 paper ["Efficient high-precision integer multiplication on the GPU"](https://doi.org/10.1177/10943420221077964) achieved 2-5× speedups, but on **NVIDIA data-center GPUs with native u64 support**, using CUDA.

### 4.3 Obstacle B — No Native 64-bit Integer Multiply

Consumer GPU ALUs are optimized for 32-bit operations. A single `u64 × u64 → u128` requires:

```
// Example: 4 u32 multiplications + carries for one u64 multiply
mul.lo.u32  lo_a, hi_b    // partial products
mul.hi.u32  ...
add.cc.u32  ...           // carry chain accumulation
```

This inflates instruction count 3-4× compared to CPU (where `mulx`/`imul` handles 64-bit natively). Combined with carry propagation, the GPU efficiency per FLOP drops dramatically.

### 4.4 Obstacle C — Dynamic Memory Management

The binary reduction tree produces operands of varying sizes:
- Level 1: 90,000 multiplications, operands ~64 bits
- Level 8: ~700 multiplications, operands ~8K bits
- Level 17: 1 multiplication, operands ~1.4M bits

Allocating variable-size buffers in VRAM for each level requires either:
- A GPU-side memory allocator (complex, high overhead per allocation)
- Pre-allocated scratch buffers (wastes VRAM, limits max `n`)

CPU heap allocation (`malloc`) is amortized O(1) and deeply optimized. GPU equivalents are not.

### 4.5 Obstacle D — PCIe Transfer and Kernel Launch Overhead

Even if all arithmetic were magically instant:
- Kernel launch latency: ~5-50 µs per dispatch
- PCIe 4.0 ×8 (RX 7600 link): ~15.75 GB/s theoretical, ~12 GB/s practical
- Transferring 360 KB (final result for n=180K): negligible
- But each intermediate result round-trip would add up

For the ~18 tree levels, if each requires a separate kernel launch, total launch overhead alone could exceed 1 ms.

### 4.6 Performance Estimation

| Scenario | CPU (current) | GPU (optimistic) | GPU (realistic) |
|---|---|---|---|
| n = 180,000 | 82.6 ms | 120-200 ms (slower) | 60-80 ms (at best parity) |
| n = 1,000,000 | ~5-10 s (estimated) | 2-4 s | 3-6 s |
| n = 10,000,000 | ~minutes | 1.5-3× faster | 1.2-2× faster |

The GPU would only pull ahead for n > 1,000,000, and even then modestly, after enormous development effort.

---

## 5. Recommendations

### 5.1 Do not pursue GPU acceleration

The problem is I/O and control-flow bound, not FLOP-bound. The RX 7600's 21.75 TFLOPS would go mostly unused due to carry propagation serialization, warp divergence, and lack of u64 hardware support.

### 5.2 High-ROI Alternatives

These all deliver better speedups with **orders of magnitude less development time**:

| Alternative | Effort | Est. speedup (n=180K) | Est. speedup (n=1M) |
|---|---|---|---|
| **`rug` (GMP bindings)** — GMP's hyper-optimized factorial in C/asm, with balanced multiplication tree + Karatsuba/Toom-Cook/FFT | 1 day | 5-10× | 10-20× |
| **Manual balanced multiplication** — instead of Rayon's generic `product()`, split range into size-balanced chunks, multiply smallest-first | 3-5 days | 1.5-3× | 3-5× |
| **`ibig` crate** — pure-Rust arbitrary precision with Karatsuba/Toom-Cook, generally faster than `num-bigint` | 1 day | 1.5-2× | 2-4× |
| **Profile + micro-optimize** — `perf`/flamegraph the current reduction to find hotspots, optimize allocation patterns | 2-3 days | 1.2-1.5× | 1.2-1.5× |
| **GPU (OpenCL)** — full implementation | 2-4 months | 0.8-1.2× (likely slower) | 1.5-3× |

### 5.3 If GPU Must Be Pursued

The only viable path is **OpenCL via the `ocl` crate**:

1. Split the factorial range into chunks on the CPU
2. Multiply each chunk to a uniform intermediate size on the CPU
3. Offload only the large N-limb multiplications to GPU kernels
4. Use carry-save representation in the kernel, one normalization pass
5. Accept that speedup will be marginal and the codebase will fork into two languages

---

## 6. Sources

- [AMD ROCm Compatibility Matrix](https://rocm.docs.amd.com/en/latest/compatibility/compatibility-matrix.html)
- [ROCm/ROCm Issue #5555 — RX 7600 gfx1102 support](https://github.com/ROCm/ROCm/issues/5555)
- [ROCm/librocdxg Issue #32 — Add gfx1102 support](https://github.com/ROCm/librocdxg/issues/32)
- [Efficient high-precision integer multiplication on the GPU](https://doi.org/10.1177/10943420221077964) — Diéguez et al., 2022
- [GPU Implementations for Midsize Integer Addition and Multiplication](https://cs.uwaterloo.ca/~smwatt/pub/reprints/2024-langcompan-gpu-arith.pdf) — Watt, 2024
- [AIM: Accelerating Arbitrary-precision Integer Multiplication on Heterogeneous Hardware](https://arxiv.org/abs/2309.12275) — Zhou et al., 2023
- [CubeCL — Multi-platform GPU Computing in Rust](https://github.com/tracel-ai/cubecl)
- [Rust GPU — Integer polyfills issue #307](https://github.com/Rust-GPU/rust-gpu/issues/307)
- [`ocl` crate — OpenCL for Rust](https://github.com/cogciprocate/ocl)
- [Framework Community — ROCm on gfx1102](https://community.frame.work/t/stable-diffusion-rocm-pytorch-setup/53802)
