# Using hooke-spring in Godot

## Foreword

Knowing how to write Rust code is ***not*** a prerequisite for using this library in your projects. The bindings are designed to integrate with the Godot Engine directly, so that **you can use it seamlessly in GDScript**.

To re-emphasize: **Just because the library is written in Rust, doesn't mean you *need* to know Rust to use it.**

```gdscript
var my_spring = RSHookeSpring.new_vector3(0.75, 3.0, RSStopwatch.new_running())
my_spring.impulse(Vector3(6.0, -8.0, 3.5))
await get_tree().create_timer(0.5).timeout
print(str(my_spring.get_position()))
```

**For more usage examples, refer to** [**here**](https://github.com/StarlightSora/hooke-spring/tree/main/examples/godot_bind).

## Installation Guide

1. [I just want to use this library in GDScript in the simplest way possible!](https://github.com/StarlightSora/hooke-spring/blob/master/README-GODOT.md#quick-install)

2. [I want to use this library in an existing Godot project that already uses godot-rust!](https://github.com/StarlightSora/hooke-spring/blob/master/README-GODOT.md#integration-with-godot-rust)

3. [I want to build the library from source code!](https://github.com/StarlightSora/hooke-spring/blob/master/README-GODOT.md#building-from-source)

## Performance

Each RSHookeSpring instance is lazily evaluated; that is, they only recalculate its position and velocity whenever it's actually necessary, saving on unnecessary computation.

Besides, the calculation itself pays very little computation time, since the library's internals are written in Rust, while still being easy to use, thanks to the Godot bindings provided.

Here is a performance test of simulating 1, 10 and 100 springs over 100 simulated seconds with a target framerate of 240FPS. You can see the source code of this test [here](https://github.com/StarlightSora/hooke-spring/blob/master/examples/godot_bind/binding_performance_test.gd).

1 `Vector3`-based `RSHookeSpring`:

```
100.0s | 24000f -> 240.0FPS (4166.666 usec/f)
calc time: 1.679 usec/f -> 0.04% of frame budget
```

10 `Vector3`-based `RSHookeSpring`s:

```
100.0s | 24000f -> 240.0FPS (4166.666 usec/f)
calc time: 17.195 usec/f -> 0.412% of frame budget
```

100 `Vector3`-based `RSHookeSpring`s:

```
100.0s | 24000f -> 240.0FPS (4166.666 usec/f)
calc time: 159.642 usec/f -> 3.831% of frame budget
```

Computation time varies between computer hardware, but as observed here, even 100 springs being evaluated every frame uses less than 5% of the target framerate's computation time budget in this test.

A native GDScript port is planned to be written in the near future. Concrete performance comparison with a native GDScript port of this library will be reported here once it is released.

## Documentation

For documentation, refer to the documentation available on [crates.io](https://docs.rs/hooke-spring/0.1.1/hooke_spring), and navigate to `godot_bind` > `rs_hooke_spring` > `RSHookeSpring` for the `RSHookeSpring` API, and `godot_bind` > `rs_stopwatch` > `RSStopwatch` for the `RSStopwatch` API.

Unfortunately a formal GDScript-specific documentation page is not available at the time of writing. However, the documentation on crates.io is still mostly applicable for GDScript usage.

## Compatibility

This library requires Godot 4.1 or newer.

## Quick Install

*Note: At the time of writing, only Windows and Linux releases are provided, due to special requirements for other builds. If you need support for Android, Mac and iOS, you need to [build the library from source](https://github.com/StarlightSora/hooke-spring/blob/master/README-GODOT.md#building-from-source), and refer to* [*chapter 5 of The godot-rust Book.*](https://godot-rust.github.io/book/toolchain/index.html)

Go to the [Releases](https://github.com/StarlightSora/hooke-spring/releases) page, find the **latest release** *(don't get the ones marked as "pre-release" unless you want a bleeding edge development build)* and grab the file named `GD_RSHookeSpringLib_xx_xx_xx.7z` (where `xx_xx_xx` is the version number).

Next, unzip the file. Using [PeaZip](https://peazip.github.io) is recommended if you're having trouble unzipping it. After unzipping, you should end up with a folder called `GD_RSHookeSpringLib`. If it's named `GD_RSHookeSpringLib_xx_xx_xx`, then the actual folder is likely *inside* it.

Drag and drop the folder inside your Godot project directory.

And you're done! Now once you open your project, you should be able to access the `RSHookeSpring` and `RSStopwatch` classes in GDScript.

## Integration with godot-rust

This assumes that you already have a Godot project that uses godot-rust. If you don't, then you should either use the [Quick Install](https://github.com/StarlightSora/hooke-spring/blob/master/README-GODOT.md#quick-install) guide instead if you have no interest in using Rust in your Godot projects, or refer to [The godot-rust Book](https://godot-rust.github.io/book/index.html)'s chapter 1, 2 and 3 first if you want to use Rust.

Add this crate to your crate's `Cargo.toml`:

```bash
cargo add hooke-spring --features godot_bind
```

Then make sure to add this to your `lib.rs` so the library gets loaded to Godot:

```rs
extern crate hooke_spring;
```

Now rebuild your crate with `cargo build`, and you're done!

Now once you open your project, you should be able to access the `RSHookeSpring` and `RSStopwatch` classes in GDScript. You should be able to access these in Rust code as well.

## Building From Source

If you want to build the dynamic library file from source, you will need the following:

- [Git](https://git-scm.com)

- [rustup](https://rustup.rs)

- If you plan to modify the source code, an IDE is strongly recommended. Example: [Visual Studio Code](https://code.visualstudio.com/download)

Before you begin, make sure all of the above are installed in your computer.

First you need to clone this repository somewhere. Open Git Bash *(Command Prompt/Terminal typically works as well)* in a directory you want to clone the project to, then run:

```bash
git clone https://github.com/StarlightSora/hooke-spring.git
```

You should see a folder called `hooke-spring` appear.

Open `Cargo.toml` inside the folder, and uncomment the lines here:
```bash
## Uncomment below two lines if building a standalone cdylib build
#[lib]
#crate-type = ["cdylib"]
```

Now go to the crate directory in Bash:

```bash
cd hooke-spring
```

and build the crate:

```bash
cargo build --release --features godot_bind
```

*The command line arguments depends on your use case, but you should always include* `--features godot_bind`.

Use `--target` to change the build target for the library. The official releases use `--target x86_64-unknown-linux-gnu` for Linux `.so`s.

Go to `hooke-spring/target/debug` (or `hooke-spring/target/release` if you used `--release`). You should see `hooke_spring.dll` (or `.so`/`.dylib` depending on your build target). This is the built dynamic library file.

Copy this file and put it in a folder in your Godot project directory. Then you need to make a `.gdextension` file to make Godot recognize the dynamic library file. You can learn more about this process in [The godot-rust Book](https://godot-rust.github.io/book/intro/hello-world.html#wire-up-godot-with-rust), it's a 5-minute read. **Make sure to set the `entry_symbol` as `"rs_hooke_spring_lib"`**, as that's the `entry_symbol` set in this library.

And you're done! Now once you open your project, you should be able to access the `RSHookeSpring` and `RSStopwatch` classes in GDScript.

### Troubleshooting

If you are having trouble building the crate for Linux on a Windows machine (especially during the linking step), check the following:

- Have you run `rustup target add x86_64-unknown-linux-gnu` before?

- Do you have Ubuntu on WSL (Windows Subsystem for Linux), and ran the command in a WSL CLI?

To install Ubuntu on WSL, run `wsl --install -d Ubuntu`. You'll also want the [WSL extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) in the VSCode marketplace (or something equivalent for your IDE).

Then open a "Ubuntu (WSL)" terminal in VSCode.

You should install the build-essential package in WSL:

```bash
sudo apt-get update && sudo apt-get install -y build-essential pkg-config clang lld
```

As well as rustup:

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y && source "$HOME/.cargo/env" && rustc --version && cargo --version
```

Finally run `cargo build --release --features godot_bind --target x86_64-unknown-linux-gnu` in the WSL terminal. It should compile without issue.

The built `.so` should be located at `/target/x86_64-unknown-linux-gnu/`.