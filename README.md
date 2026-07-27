# hooke-spring: A Lightweight Spring Simulation API in Rust
![no_std](https://img.shields.io/badge/no__std-✓-blue)

## Feature Flags
- Default: `std` + `smol_stopwatch`
- `std`: Use the `std` crate. Incompatible with `no_std`.
- `smol_stopwatch`: Provides `SmolStopwatch` for real‑time clocks. Will auto-enable `std`. Incompatible with `no_std`.
- `no_std`: Do not depend on the the `std` crate. Will auto-enable `libm`. Incompatible with `std`.
- `libm`: Use the `libm` crate for math operations, instead of `f64` intrinsics. If enabled alongside `std`, then `libm` takes priority for math operations.

If your project needs `no_std` + `alloc`, use `--no-default-features --features no_std`.

If you do not need `SmolStopwatch`, you can use `--no-default-features --features std`.

## Attribution
This project is a Rust rewrite inspired by the `Spring.lua` module from [NevermoreEngine](https://github.com/Quenty/NevermoreEngine).

To view the license of the original library, see `/LICENSES/LICENSE-NevermoreEngine.txt`