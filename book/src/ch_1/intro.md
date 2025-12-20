
This is an embedded-rust tutorial using the esp32-c6 board.

Rust knowledge is assumed, but no embedded experience is required. The content was written for Unix (Linux, MacOS).

The book aims to be self-contained, but short [reading material](#suggested-reading) will be suggested. If the license allows for, we will copy useful content here, and provide the source.

## Required Hardware

 The esp32-c6 board is available on Mouser, Aliexpress and other retailers. It should look similar to:

<div class="center w220">
    <a href="/assets/board.jpg"> <img  alt="Close up image of our board." src="/assets/board.jpg"/>
    </a>
</div>

Other required hardware:

- USB-C cable to link your computer and the board.

    - The cable *has to* support data transfers and not be power-only.

- One resistor, 2 jumper wires, one LED, one breadboard. These are needed for *Blinky* and *Handle Input* chapters.

## Workshop repository

The source for the book and code-exercises is at <https://github.com/iampi31415/iampi31415.github.io>.
To get started, clone repo and then change directory:

    git clone https://github.com/iampi31415/iampi31415.github.io
    cd iampi31415.github.io

**Repository contents**. The directory contains:

- `book/`: markdown sources of this book
- `exercises/`: code examples and exercises

## Suggested reading

- esp32-c6 [user guide]: you can find the labelled components, list of peripherals, and schematics.

## Related resources

- [Awesome ESP Rust]
- [esp-rs community] on Matrix

<!--Links used in this document-->
[Awesome ESP Rust]: https://github.com/esp-rs/awesome-esp-rust
[user guide]: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/user_guide.html
[esp-rs community]: https://matrix.to/#/#esp-rs:matrix.org

<!-- Too much info. May be OK for an appendix.
* [Code examples] for setting up peripherals
* [MCU datasheet] i.e for the esp32-c6 micro controller.
* [reference manual] for esp32-c6 board: large, complete description of
board.
[MCU datasheet]: https://documentation.espressif.com/esp32-c6-wroom-1*wroom-1u*datasheet_en.pdf

[reference manual]: https://documentation.espressif.com/esp32-c6*technical*reference*manual*en.pdf

[Code examples]: https://github.com/esp-rs/esp-hal/tree/main/examples
-->
