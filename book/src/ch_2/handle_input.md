
Using same wiring from the `blinky` chapter, let's now use external input to make the LED blink.

- Access `exercises/handle_input` in order to edit `src/main.rs`.

## Exercise

We will use the button labelled [`BOOT` linked to `GPIO9`][Button-GPIO9] to toggle the LED.

1. Initialise the peripherals with default config (same as previous exercises).
2. Create an `Output::new` passing `GPIO7` peripheral, `Level::High` and default `OutputConfig`.
    - Assign it to the `led` variable.
    - Hint: with _peripherals_ initialised, `GPIO7` is a field in that struct.
3. Create an `Input::new` passing `GPIO9` peripheral, default `InputConfig` but overwrite with `as_pull(Pull::High)`.
    - Assign it to the `btn` variable.
4. Add the logic inside `loop`:
   - When pressing it should turn the `led` on, and delay it 2 seconds.
   - Then it turns itself off.

The `examples/handle_input.rs` contains a solution. You can run it with the following command `cargo run --example handle_input --release`.

[Button-GPIO9]: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/user_guide.html#id15
