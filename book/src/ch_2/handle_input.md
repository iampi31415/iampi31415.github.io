
Using same wiring from the `blinky` chapter, let's now use external input to make the LED blink.

- Access `exercises/handle_input` in order to edit `main.rs`.

## Exercise

We will use the button labelled [`BOOT` linked to `GPIO9`][Button-GPIO9] to toggle the LED.

1. Initialise the peripherals with default config (check previous exercises)
2. Set the `led` variable to `Output::new` passing `GPIO7` peripheral, `Level::High` and default `OutputConfig`.
3. Set the `btn` variable to `Input::new` passing `GPIO9` peripheral, default `InputConfig` but overwrite with `as_pull(Pull::High)`.
4. Add the logic inside `loop`
   - When pressing it should turn the `led` on, and delay it 2 seconds
   - Then it turns itself off.

The `exercises/handle_input/examples/handle_input.rs` contains a solution. You can run it with the following command `cargo run --example handle_input --release`.

[Button-GPIO9]: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/user_guide.html#id15
