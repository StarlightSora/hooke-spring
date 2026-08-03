# Using hooke-spring in Godot

## Foreword

Knowing how to write Rust code is ***not*** a prerequisite for using this library in your projects. The bindings are designed to integrate with the Godot Engine directly, so that you can use it seamlessly in GDScript.

To re-emphasize: **Just because the library is written in Rust, doesn't mean you *need* to know Rust to use it.**

**If you know how to write some Rust code**, and understand the Rust ecosystem to a basic level, **that's great!** You can get started by reading the [Getting Started Fast Guide](https://github.com/StarlightSora/hooke-spring/blob/master/README_GODOT.md#getting-started-fast-guide).

**If you have absolutely no experience**, or very little experience with Rust, **don't worry!** You can get started by reading the [Getting Started From Zero Guide](https://github.com/StarlightSora/hooke-spring/blob/master/README_GODOT.md#getting-started-from-zero-guide).
Note that this guide *does not* cover writing Rust code in general (again, you don't *need* to know Rust if you just want to use the library!); it only provides the necessary code to use this library to your Godot projects. If you're interested in writing Rust code in general, refer to [The Rust Book](https://doc.rust-lang.org/stable/book/title-page.html).

For more information about exporting your Godot projects with godot-rust (especially for mobile and Mac), refer to [chapter 5 of The godot-rust book](https://godot-rust.github.io/book/toolchain/index.html).

If you want a more comprehensive guide for using Rust for Godot development in general, refer to [The godot-rust Book](https://godot-rust.github.io/book/index.html) and the [godot-rust docs](https://godot-rust.github.io/docs/gdext/master/godot/index.html). The guide here is adapted from this book's second chapter.

## Adding to An Existing Project Already Using godot-rust

If you have an existing Godot project that already uses godot-rust, simply add this crate to your Cargo.toml dependencies:
```bash
cargo add hooke-spring --features godot_bind
```
Then add this to your `lib.rs`:
```rs
extern crate hooke_spring;
```

Then run `cargo build`. You should be able to see the `RSHookeSpring` and `RSStopwatch` class in GDScript.

## Getting Started Fast Guide

Create a new library crate in your Godot project directory. This guide will assume the project structure below. The crate name will be `gdrs` for this example.

To make a new library crate, open Terminal/Command Prompt (or something equivalent in your OS) in your project folder, and run: `cargo new gdrs --lib`
```
📁 my-godot-project
├─ 📁 .git
├─ 📄 gdrs_link.gdextension // We will get to this soon
└─ 📁 gdrs
   ├─ 📄 .gdignore
   ├─ 📁 src
   │  └─ 📄 lib.rs
   └─ 📄 Cargo.toml
```
Note: The `.gdignore` file is an empty file with an empty name and the file extension `.gdignore` so Godot doesn't scan this directory for discovering game assets. It is highly recommended to add this to your crate's root directory so Godot doesn't try to incorrectly parse `.obj` files from running `cargo build`. You don't need this if your crate is located outside your Godot project directory.

*Note: If your Godot project uses Git for version control, and you made the crate inside your project (this guide does so), you should run `git submodule add /gdrs` afterwards to add this as a submodule of your project.*

Open `Cargo.toml` and add these:
```toml
[lib]
crate-type = ["cdylib"] # Compile this crate to a dynamic C library.

[dependencies]
godot = "0.5.4"
hooke-spring = { features = ["godot_bind"] }
```
Now run `cargo build` to build the crate. Make sure you run this command in your crate's directory (`my-godot-project/gdrs`), not your actual Godot project directory (`my-godot-project`)! You may need to run `cd gdrs` to navigate to your crate.

Now create a file named `gdrs_link.gdextension` in your Godot project directory. (You can name `gdrs_link` to whatever else you want.) Open it, and paste this in:
```toml
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
reloadable = true

[libraries]
linux.debug.x86_64 =     "res://gdrs/target/debug/gdrs.so"
linux.release.x86_64 =   "res://gdrs/target/release/gdrs.so"
windows.debug.x86_64 =   "res://gdrs/target/debug/gdrs.dll"
windows.release.x86_64 = "res://gdrs/target/release/gdrs.dll"
macos.debug =            "res://gdrs/target/debug/gdrs.dylib"
macos.release =          "res://gdrs/target/release/gdrs.dylib"
macos.debug.arm64 =      "res://gdrs/target/debug/gdrs.dylib"
macos.release.arm64 =    "res://gdrs/target/release/gdrs.dylib"
```

`compatibility_minimum`: You can set this to whatever version your project is as long as it's at least `4.1`.
`libraries`: Rename `gdrs` to whatever your crate name actually is. For example, if your crate name is `my-crate`, then it should be something like `res://my-crate/target/debug/my-crate.dll`. If your crate is outside your Godot project directory, you'll need to use `..` to access the parent folder, then locate the crate. For example: `res://../gdrs/target/debug/gdrs.dll`

After that, open `lib.rs` in your crate and paste this in:
```rs
use godot::prelude::*;

extern crate hooke_spring; // needed to import the library

struct MyExtension; // You can name this struct whatever you want

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
```

Run `cargo build` again. *Note: The use of `unsafe` here is necessary because godot-rust needs to communicate with Godot Engine via a FFI. Further information about this is beyond the scope of this guide.*

Open your project in Godot. You should be able to see the `RSHookeSpring` and `RSStopwatch` class in GDScript.


## Getting Started From Zero Guide

First and foremorst, it's recommended to have Git installed. If you don't, you can get it [here](https://git-scm.com).

Second, you need rustup. This is the easiest way to install the Rust toolchain, containing everything commonly needed to work on and build Rust projects. You can get it [here](https://rustup.rs).

Make a new Godot project. To do this, open Godot Engine and click Create at the top right. (You can also use an existing project you have instead.) For this guide we will have our project named as `My Godot Project` (folder name `my-godot-project`).

Go to the directory of your project. For example, if your project was created at `D:/GodotProjects/my-godot-project`, navigate there via File Explorer (or whatever equivalent to your OS).

This guide will assume this project structure:
```
📁 my-godot-project
├─ 📁 .git
├─ 📄 gdrs_link.gdextension // We will get to this soon
└─ 📁 gdrs // Should be made once you run the command mentioned below
   ├─ 📄 .gdignore // We will have you make this file soon
   ├─ 📁 src
   │  └─ 📄 lib.rs
   └─ 📄 Cargo.toml
```

Once you're in the folder (`my-godot-project`), right click on empty space and click "Open in Terminal" (or something equivalent in your OS).
Type this command and press enter:
```bash
cargo add gdrs --lib
```

*Note: If your Godot project uses Git for version control, you should run `git submodule add /gdrs` afterwards to add this as a submodule of your project.*

This should create a new folder named `gdrs`. It should have a file named `Cargo.toml`, and a folder named `src` that contains `lib.rs`, alongside some other things. This is called a crate in the Rust ecosystem.

In your crate's root directory `my-godot-project/gdrs`, add a new file named `.gdignore`. This will make Godot exclude the crate when scanning for game asset files. (**Make sure you can see file extensions in File Explorer.** In Windows 11, you can enable this by going to View > Show > File name extensions)

Now open `Cargo.toml` in your crate and add these:
```toml
[lib]
crate-type = ["cdylib"] # Compile this crate to a dynamic C library.

[dependencies]
godot = "0.5.4"
hooke-spring = { features = ["godot_bind"] }
```

Run `cargo build` in the crate directory.
If you still have the terminal window open from earlier, you can do `cd gdrs` to navigate to the crate directory, then you can run `cargo build`.
If you closed it already, you can right click within the crate directory and click "Open in Terminal". Then you can run `cargo build`.

Now create a file named `gdrs_link.gdextension` in your Godot project directory (`my-godot-project`). Open it, and paste this in:
```toml
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
reloadable = true

[libraries]
linux.debug.x86_64 =     "res://gdrs/target/debug/gdrs.so"
linux.release.x86_64 =   "res://gdrs/target/release/gdrs.so"
windows.debug.x86_64 =   "res://gdrs/target/debug/gdrs.dll"
windows.release.x86_64 = "res://gdrs/target/release/gdrs.dll"
macos.debug =            "res://gdrs/target/debug/gdrs.dylib"
macos.release =          "res://gdrs/target/release/gdrs.dylib"
macos.debug.arm64 =      "res://gdrs/target/debug/gdrs.dylib"
macos.release.arm64 =    "res://gdrs/target/release/gdrs.dylib"
```

After that, open `lib.rs` in your crate (`my-godot-project/gdrs/src/lib.rs`) and paste this in:
```rs
use godot::prelude::*;

extern crate hooke_spring; // needed to import the library

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
```

Run `cargo build` in your crate directory again.

Open your project in Godot. You should be able to see the `RSHookeSpring` and `RSStopwatch` class in GDScript.