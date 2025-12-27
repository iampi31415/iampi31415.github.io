
There is a lot to learn, so let's start simple.

1. Connect the board to your computer and access the first project:

    ```bash
    cd exercises/hello_world
    ```

2. Build, flash, and monitor the project with `cargo run`.
    For now, ignore the warning about `--release`. You should see:

    ```txt
    ..
    Commands:
        CTRL+R    Reset chip
        CTRL+C    Exit
    ..
    loop..!
    ..
    ```

    Press `CTRL+C` to exit.

    The `loop..!` logs come from [esp-println]. They provide us with `dbg!`, `print!` and `println!`.

## *Build, flash and monitor*?

The `.cargo/config.toml` includes this config:

```toml
[target.riscv32imac-unknown-none-elf] # our processor arch.
runner = "espflash flash --monitor"   # for `cargo run`.
```

Our `cargo run` is replaced by `cargo build && espflash flash <elf_path> --monitor`.

> [!NOTE]
> This builds and flashes the binary. Then monitors for any logs.
> Without the `--monitor` it flashes and exits instead of waiting and printing logs.

## Exercise

`esp-println` supports backends `log` and `defmt` which provide `info!`, `warn!` and other macros. Let's try adding them:

1. Uncomment the "log lines" in `src/main.rs`,
2. Uncomment them in `Cargo.toml` as well, to add our `log` dependency.
3. The `log` crate logging level is controlled with `ESP_LOG` under the `[env]` section in `.cargo/config.toml`.
   - Change the `ESP_LOG` variable to turn `off` all logs. Re-run `cargo run --release`, to test how it works.

   - Try with other levels, for example, with `trace`.

The `examples/hello_world.rs` contains a solution.

You can run it with the following command `cargo run --release --example hello_world`.

> [!NOTE]
> Running the solution requires fixing the log-lines at the bottom of `Cargo.toml`.

## Suggested Reading

- [esp-println]
- [log] this one you can just peek at to have a general idea.

[esp-println]: https://docs.rs/esp-println/latest/esp_println/
[log]: https://docs.rs/log/latest/log/
