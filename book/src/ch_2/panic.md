
If something goes very wrong, a program will `panic!`.

[esp-backtrace] provides a `panic-handler` feature that handles `panic!`.

"Handling panic!" here means to print what was executed up to that point (the backtrace).

> [!NOTE]
> We need to `use esp-backtrace as _` so that this handler is included in the final binary.

In the exercise below, the word _profile_ is used. Profiles are configurations `cargo` uses to control compilation. When unspecified, `cargo` sets sensible defaults for us.

Common profile settings we will change are:

- Controlling binary size,
- Including debug information.

## Exercise: part 1

0. Let's access the project at `exercises/panic`.
1. In `src/main.rs` import (or _use_) the `esp-backtrace` crate.
2. Add a `panic!` somewhere, e.g. after our `println`.
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

`examples/panic.rs` contains a solution. It can be run with: `cargo run --example panic --release`.

## Recap

- Using `esp-backtrace` to handle panic and printing backtraces.
- Using `panic!` to exit the program.
- Configuring compilation so that panics print a backtrace with all debug information.

### Suggested Reading

- [esp-backtrace]

[esp-backtrace]: https://docs.rs/crate/esp-backtrace/latest
