
## Board, components and peripherals

The *board*, as shown earlier, is:
<div class="center w220">
    <a href="/assets/board.jpg"> <img  alt="Close up image of our board." src="/assets/board.jpg"/>
    </a>
</div>

The *board* then, is the whole item. Its main parts are:

- The *printed circuit board* (PCB): the flat fiberglass sheet underneath it all.
- The *components*: modules on top of the PCB. They interconnect through *traces* which are thin copper wires. Examples of components: MCU, USB ports LEDs, Pins.
    - The *microcontroller* or microcontroller unit (MCU): the big square labelled ESP32-C6-WROOM (or some variant). It takes almost half the board. The MCU includes peripherals, CPU and RAM.
        - The *peripherals* are parts of the MCU including: GPIO, UART, I2C, SPI, WiFi, Bluetooth and others. We can't see them directly.

An explanation of how it all comes together is in the [micro:bit v2][MB2] book:

> Our MCU[^1] has 73 tiny metal pins sitting right underneath it (...). These pins are connected to traces, the little "roads" that act as the wires connecting components together on the board. The MCU can dynamically alter the electrical properties of the pins. This works similarly to a light switch, altering how electrical current flows through a circuit. By enabling or disabling electrical current to flow through a specific pin, an LED attached to that pin (via the traces) can be turned on and off.

[^1]: Refers to a different MCU than ours but the concept is the same.

## What is no_std?

This tutorial is "`no_std`", but what does it mean? `std` can refer to either:

- `std`: the crate, or
- `std`: the `rust-std` component, including the crates `std`, `core` and `alloc`.

This book distinguishes these by adding the words *crate* and *component*, respectively.

The microcontroller has no Operative System, but `std` *crate* relies on one. Hence, `#[no_std]` is used in `main.rs` to indicate that `std` crate should be excluded.

To be precise, only `core` is included by default. A comparison is provided by this [the table][Rust Embedded Guide] below, copied from the Rust Embedded Guide:

| feature                                                   | no\_std | std |
|-----------------------------------------------------------|---------|-----|
| heap (dynamic memory)                                     |    *    |  ✓  |
| collections (Vec, BTreeMap, etc)                          |   **    |  ✓  |
| stack overflow protection                                 |    ✘    |  ✓  |
| runs init code before main                                |    ✘    |  ✓  |
| libstd available                                          |    ✘    |  ✓  |
| libcore available                                         |    ✓    |  ✓  |
| writing firmware, kernel, or bootloader code              |    ✓    |  ✘  |

\* Only if you use the `alloc` crate and use a suitable allocator like [esp-alloc].

\** Only if you use the `collections` crate and configure a global default allocator.

\** HashMap and HashSet are not available due to a lack of a secure random number generator.

[MB2]: https://docs.rust-embedded.org/discovery-mb2/04-meet-your-hardware/microbit-v2.html
[Rust Embedded Guide]: https://docs.rust-embedded.org/book/intro/no-std.html#overview
[esp-alloc]: https://docs.rs/crate/esp-alloc/latest
