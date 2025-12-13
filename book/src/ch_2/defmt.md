
[`defmt`][defmt] is an efficient logging framework. Just like `log`, the `defmt` crate enables `defmt::info`, `defmt::warn` and other levels. It also has a `defmt::println!` macro.

The name `defmt` comes from *deferred formatting*:
> (...) instead of formatting `255u8` into `"255"` and sending the string, the single-byte binary data is sent to a second machine, the host, and the formatting happens there.

So the formatting is *deferred* to the host. The other bit improving efficiency is *compression*:

> `defmt`’s string compression consists of building a table of string literals, like `"Hello, world"` or `"The answer is {:?}"`, at compile time. At runtime the logging machine sends *indices* instead of complete strings.

Source: [defmt docs].

## `defmt` Ecosystem

[`esp-println`][esp-println], [`esp-backtrace`][esp-backtrace] and [`espflash`][espflash] provide mechanisms to use `defmt`:

- `espflash` has support for different [logging formats], one of them being `defmt`.

- `esp-println` needs `defmt-espflash` feature. As [their docs][esp-println] state:
    > Using the `defmt-espflash` feature, `esp-println` will install a `defmt` global logger. Updated the `Cargo.toml`'s features.
- `esp-backtrace` needs a `defmt` feature that uses `defmt` logging to print panic and exception handler messages.

## Exercise

Go to `exercises/defmt` directory.

1. Update the runner's logger in `.cargo/config.toml`:

    ```diff
    [target.riscv32imac-unknown-none-elf]
    - runner = "espflash flash --monitor"
    + runner = "espflash flash --monitor -L defmt"
    ```

    so that `espflash` monitor can decode the format of the `defmt` messages received.

2. Update `Cargo.toml` to include the needed features:

    ```diff
    esp-println = { version = "0.16.0", features = [
        "esp32c6",
    -     "log-04",
    +     "defmt-espflash",
    ] }
    # (...)
    esp-backtrace = { version = "0.18.0", features = [
        "esp32c6",
        "panic-handler",
    +   "defmt",
    ]}
    ```

3. Due to the [linking process] we need to add `defmt` linker script to `cargo/config.toml`:

    ```diff
    rustflags = [
      # ....
    +  "-C", "link-arg=-Tdefmt.x",
    ]
    ```

4. Add [defmt](https://crates.io/crates/defmt) to the dependencies.
5. **Logging level**: Use the `defmt::println!` and some [defmt macros] to print a few messages.
    - When building the app, set [`DEFMT_LOG`][DEFMT_LOG] level as done for `ESP_LOG` earlier.

    - An alternative to changing `.cargo/config.toml` is using `DEFMT_LOG=<value> cargo run --release`; the same is valid for `ESP_LOG`.
6. Add a `panic!` macro to trigger a panic with a `defmt` message.

`exercises/defmt/examples/defmt.rs` contains a solution. You can run it with the following command: `cargo run --example defmt --release`. You will need to have the settings above done correctly though!

## Suggested reading

Short articles that give more context:

- defmt [linking process] for setting the compilation-time linker up
- defmt [DEFMT_LOG] environment variable
- esp-println [logging formats]

<!--esp links-->
[esp-println]: https://github.com/esp-rs/esp-hal/tree/main/esp-println
[esp-backtrace]: https://github.com/esp-rs/esp-hal/tree/main/esp-backtrace
[espflash]: https://github.com/esp-rs/espflash
[logging formats]: https://github.com/esp-rs/espflash/blob/main/espflash/README.md#logging-format

<!--defmt links-->
[defmt macros]: https://docs.rs/defmt/latest/defmt/#macros
[defmt]: https://docs.rs/defmt/latest/defmt/
[defmt docs]: http://defmt.ferrous-systems.com/introduction#operating-principle
[DEFMT_LOG]: https://defmt.ferrous-systems.com/filtering.html?highlight=DEFMT*LOG#defmt_log

[linking process]: https://defmt.ferrous-systems.com/setup#linker-script
