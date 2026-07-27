# hooke-spring
*A Lightweight Spring Simulation API in Rust*

![no_std](https://img.shields.io/badge/no__std-✓-blue)
![crates_io](https://img.shields.io/crates/v/hooke-spring)
![docs](https://img.shields.io/docsrs/hooke-spring)

## Overview
`hooke-spring` provides a simple, fast and flexible spring simulator based on Hooke’s Law. It’s designed for recoil, camera shake, UI animations, and other physically‑inspired effects. The crate is **`no_std` + `alloc` compatible**, with optional `std` features for real‑time clocks.

## Features
|Feature|Description|Requires|Incompatible With|
|---|---|---|---|
|`std`|Enables usage of `std`|(None)|`no_std`|
|`smol_stopwatch`|Provides `SmolStopwatch` for real-time|`std`|`no_std`|
|`no_std`|Disables usage of `std`, use `libm` + `alloc` instead|`libm`|`std`|
|`libm`|Uses the `libm` crate for math instead of `f64` intrinsics|(None)|(None)|

## Usage
```bash
# Default (std + smol_stopwatch)
cargo add hooke-spring

# no_std + alloc
cargo add hooke-spring --no-default-features --features no_std

# std without SmolStopwatch
cargo add hooke-spring --no-default-features --features std
```

## Example
```rs
use hooke_spring::{HookeSpring, SmolStopwatch};
use std::{time, thread};

let mut spring = HookeSpring::<f64>::from_damper_speed(0.75, 8.0, Some(SmolStopwatch::wrapped()));
spring.impulse(10.0);
for i in 0..10 {
    let (pos, vel) = spring.get_position_and_velocity();
    println!("[Iteration {i}] Position: {}, Velocity: {}", pos, vel);
    thread::sleep(time::Duration::from_millis(100));
}
```

## Roadmap

- **0.1**: Core spring simulation, `no_std` support, integrated time-keeping structs
- **0.2**: Bindings for `godot-rust` for integration with Godot (Will be opt-in via features)


## Attribution
This project is a Rust rewrite inspired by the `Spring.lua` module from [NevermoreEngine](https://github.com/Quenty/NevermoreEngine).

To view the license of the original library, see `/LICENSES/LICENSE-NevermoreEngine.txt`