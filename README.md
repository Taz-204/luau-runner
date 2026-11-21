# Luau Runner

A small Windows desktop app for writing and running Luau scripts. Type code in the editor, press **Execute**, and see the printed output and any errors.

## Features

- Code editor with a monospace font
- Runs scripts in a fresh Luau VM each time
- `print` output appears in the output panel
- A 2-second timeout stops runaway loops

## Build

Requires Rust (https://rustup.rs) and, on Windows, the Visual Studio C++ build tools.

```
cargo run              # run in a window
cargo build --release  # produces target/release/luau-runner.exe
```

The GitHub Actions workflow in `.github/workflows/build.yml` also builds the `.exe` on every push to `main`. Download it from the run's artifacts.

## Notes

- Built on [mlua](https://crates.io/crates/mlua) with the Luau backend and [egui](https://github.com/emilk/egui) for the UI.
- Unsigned builds may show a SmartScreen warning on first run.
