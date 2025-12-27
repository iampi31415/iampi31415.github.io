
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

`esp-println` supports backends `log` and `defmt` which provide `info!`, `warn!` other macros. Let's try adding them:

1. Uncomment the lines in `src/main.rs` and in `Cargo.toml` to enable `log`.
2. The `log` crate logging level is controlled with `ESP_LOG` under the `[env]` section in `.cargo/config.toml`.
    - Change the `ESP_LOG` variable to turn `off` all logs. Re-run `cargo run --release`, to test how it works.

    - Try with other levels, for example, with `trace`.

The `exercises/hello_world/examples/hello_world.rs` contains a solution.

You can run it with the following command `cargo run --release --example hello_world`. You should first fix the lines at the bottom of `Cargo.toml`.

## Suggested Reading

- [esp-println]
- [log] this one you can just peek at to have a general idea.

[esp-println]: https://docs.rs/esp-println/latest/esp_println/
[log]: https://docs.rs/log/latest/log/
