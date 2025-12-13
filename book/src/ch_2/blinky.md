# Blinky

We will now create the iconic _blinky_.

Let's access the project with `cd exercises/blinky`. We will need to edit the file `main.rs`.

On esp32-c6 board there is no regular LED connected, instead there is an addressable LED which works differently and is beyond the scope of this book.

Instead, we will use a regular LED and a resistor, and build a circuit controlled with the GPIO pin headers.

<div class="center w320">
<a href="/assets/led_connection.jpg">
<img alt="Wire up a circuit with an LED and a resistor connected to the board." src="/assets/led_connection.jpg"/>
</a>
<p>esp32-c6, wiring the LED</p>
</div>

Wire up the board as shown on the previous image:

1. Start wiring from `GND` pin header (red wire),

2. From there to the resistor (220mΩ or otherwise, without it the LED blows up.)
3. From the other leg of the resistor to the LED (blue wire),

4. Finally, the LED connects to GPIO7 (the long LED-leg is on GPIO7.)

## Exercise

1. Create `OutputConfig` with default configuration.
    - Hint: it implements `Default`.
2. Toggle the `led` with 3500ms delay.

The `exercises/blinky/examples/blinky.rs` contains a solution.

You can run it with the following command `cargo run --example blinky --release`.
