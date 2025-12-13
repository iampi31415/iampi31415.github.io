
Let's install all we need.

## Rust Tools

1. Install Rust Toolchain following the steps in <https://rustup.rs/>
    - The Rust Toolchain is `rustup` plus many components `cargo`, `rustc`, the *compiled* `std` component and so on.
2. We also want the compiled `std`-component for our target microcontroller

    ```bash    
    rustup target add riscv32imac-unknown-none-elf
    ```

3. Other development components:

    ```bash
    rustup component add rust-analyzer rust-src 
    ```

    `rust-src` is the `std`-component source code, useful for the `rust-analyzer`.

## Espressif tools

[`espflash`](https://github.com/esp-rs/espflash/tree/main/espflash) is used for flashing our ELF binary (our program) into the board.

Install it with: `cargo install espflash`

## Build dependencies

- Debian/Ubuntu.

    ```bash
    sudo apt install llvm-dev libclang-dev clang
    ```

- MacOS. When using the Homebrew package manager, which we recommend:

    ```bash
    brew install llvm
    ```

## Additional Software

- Editor: Neovim, Zed, VSCode, Helix.
- [`Rust Analyzer`](https://rust-analyzer.github.io/) LSP, for code completion, formatting.
- `Even Better TOML` LSP, for editing TOML based configuration files

<!--

////// This is too advanced and kind of in detriment of learning
////// So I am commenting it out for now.

## Aside

We could install `nightly` in step `1.`; then `2.` and `3.` work the same.

But with `nightly` we can skip step `2.` and use modified `3.`

```bash
rustup component add rust-src rustfmt clippy rust-analyzer
```

`rust-src` is the *source code* of the standard library component.

Then, for each exercise/crate, we need to update `.cargo/config.toml` with:

```diff
+[unstable]
+build-std=["core"]
```

We also need to modify the `rust-toolchain.toml`:

```diff
[toolchain]
+channel = "nightly"
-channel = "stable"
components = ["rust-src"]
-targets = ["riscv32imac-unknown-none-elf"]
```

When running `cargo run --release` core will be compiled along with our crate (not used the precompiled one). However, this is *unstable* and it is only mentioned here as a cool feature.
-->
