# hooke-spring
*A Lightweight Spring Simulation API in Rust*

![no_std](https://img.shields.io/badge/no__std-✓-blue)
![crates_io](https://img.shields.io/crates/v/hooke-spring)
![docs](https://img.shields.io/docsrs/hooke-spring)
![lstcmt](https://img.shields.io/github/last-commit/StarlightSora/hooke-spring)

## Overview
`hooke-spring` provides a simple, fast and flexible spring simulator based on Hooke’s Law. Although originally designed for use in games, it can be used wherever a lightweight spring simulator is needed. The crate is **`no_std` + `alloc` compatible**, with optional `std` features for real‑time clocks. As of 0.2.0, **Godot 4 support is available right out of the box** with the `godot_bind` feature flag.

**For usage in Godot**, refer to [this guide](https://github.com/StarlightSora/hooke-spring/blob/master/README_GODOT.md).

## Use Cases
You can use this library for:

- Simulating weapon recoil
- Shaking the camera for player feedback
- Smoothing transitions for moving objects and procedural animations
- Animating UI elements
- Smoothing out value changes for displaying on a UI

... and many more!

## Flags
This crate has the following Cargo features:
|Feature|Description|Requires|Incompatible With|
|---|---|---|---|
|`std`|Enables usage of `std`|(None)|`no_std`|
|`smol_stopwatch`|Provides `SmolStopwatch` for real-time|`std`|`no_std`|
|`no_std`|Disables usage of `std`, use `libm` + `alloc` instead|`libm`|`std`|
|`libm`|Uses the `libm` crate for math instead of `f64` intrinsics|(None)|(None)|
|`godot_bind`|Enable bindings for Godot Engine|(None)|(None)*|

The crate enables `std` and `smol_stopwatch` by default.
\*The `godot` crate itself doesn't work in a `no_std` environment.

## Usage
Add the crate to your `Cargo.toml`:
```bash
# Default (std + smol_stopwatch)
cargo add hooke-spring

# no_std + alloc
cargo add hooke-spring --no-default-features --features no_std

# std without smol_stopwatch
cargo add hooke-spring --no-default-features --features std

# For use in Godot
cargo add hooke-spring --features godot_bind
```

## Example
```rs
use hooke_spring::{HookeSpring, SmolStopwatch};
use std::{time, thread};

let mut spring = HookeSpring::<f64>::from_damper_speed(0.75, 8.0, Some(SmolStopwatch::wrapped()));
spring.impulse(10.0);
for i in 1..=10 {
    let (pos, vel) = spring.get_position_and_velocity();
    println!("[Iteration {i}] Position: {}, Velocity: {}", pos, vel);
    thread::sleep(time::Duration::from_millis(100));
}
```

## Roadmap
- **0.1**: Core spring simulation, `no_std` support, integrated time-keeping structs
- **0.2**: Integration for game engines (will be opt-in via features)
  - Integration with Godot via [godot-rust](https://godot-rust.github.io/) bindings
  - ~~Integration with Bevy~~ *Deferred to 0.3*
- **0.3**: Integration with Bevy's data types

## Attribution
This project is licensed under the MIT license. See [here](https://github.com/StarlightSora/hooke-spring/blob/master/LICENSE.txt) for more information.

This project is a Rust rewrite inspired by the [Spring.lua](https://github.com/Quenty/NevermoreEngine/blob/main/src/spring/src/Shared/Spring.lua) module from the [NevermoreEngine](https://github.com/Quenty/NevermoreEngine) library.
To view the license of the original library, see [here](https://github.com/StarlightSora/hooke-spring/blob/master/LICENSES/LICENSE-NevermoreEngine.txt).