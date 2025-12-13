# Panic

If something goes wrong, the program will `panic!`. In embedded, we will always need to add a `panic-handler`.

[esp-backtrace] provides a `panic-handler` feature that handles `panic!`. This is why we need to `use esp-backtrace as _`.

Let's access the project at `exercises/panic`, and modify the code to test `panic!`.

We will also take a look at compilation _profiles_. We don't need deep knowledge about profiles, just the very basics.

`cargo` uses profile-settings to control compilation using defaults when unspecified. We are interested in controlling a few aspects:

- Should it be as fast as possible? As short as possible?
- Should it include information useful for debugging?

## Exercise: part 1

1. In `main.rs` use the `esp-backtrace` crate.
2. Then add a `panic!` somewhere, e.g. after our `println`.
3. Run the code with `cargo run`; this uses the _development profile_.

    - It outputs debug information into the compiled binary.
4. Then run with _release profile_ `cargo run --release`.

    - This profile will not output all debug information in the binary. We should find:

        ```console
        Hello world!
        ====================== PANIC ======================
        panicked at examples/panic.rs:24:5:
        This is a panic
        Backtrace:
        0x4200252a
        main
            at ??:??
        ```

        The default `--release` behaviour _excludes_ debug information and minimises the binary size; the backtrace shows the missing debug information with `??`.

## Exercise: part 2

We nearly always use `--release` (we want small binary size). If we want debug information we need to configure the release profile in `.cargo/config.toml`:

```diff
+[profile.release]
+debug = true
```

Now it will emit debug information in the ELF binary file; _yet debug info isn't flashed_ into the target, it is just used to display the backtrace.

- Re-run the program with `--release` and confirm `??:??` is now filled in.

`exercises/panic/examples/panic.rs` contains a solution. It can be run with: `cargo run --example panic --release`.

## Recap

- `esp-backtrace` handles panics (by us or by any library).
- We can manually use `panic!` to exit the program.
- Configure compilation so that panics print a backtrace with all debug information.

### Suggested Reading

- [esp-backtrace]

[esp-backtrace]: https://docs.rs/crate/esp-backtrace/latest
