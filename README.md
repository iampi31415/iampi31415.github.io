# Rust Embedded Basics (esp32-c6)

**For a few days the repo won't accept PRs until it's more stable.**

But issues are welcome.

Learn the basics of Embedded programming in Rust _with a esp32-c6 board_.

The idea of this book is to have both conceptual explanations and exercises.

The exercises are initially quite structured, then become more free-form.

## Contributions

Please see [Contributing.md](./CONTRIBUTING.md).

## Exercises

There are two options:

- Clone the repo to use the exercises:

    ```bash
    git clone https://github.com/iampi31415/iampi31415.github.io
    ```

- Visit the [repository](<https://github.com/iampi31415/iampi31415.github.io>

), and click the green "Code" button and then the "Download Zip" one.

The exercises are under `exercises/`.

## Book

The book is at <https://iampi31415.github.io>.

It can also be built locally with. This requires:

1. `cargo install mdbook@0.5.1` and

2. `mdbook serve --open` to build and serve.

To download the book _without_ the need to compile:

- Clone the `gh-pages` branch:

    ```bash
    git clone -b gh-pages git@github.com:iampi31415/iampi31415.github.io
    ```

    - Now you've a copy of the website
    - Click `index.hmtl` to open it.
- Go to the book URL <https://iampi31415.github.io>

    - Click "Print" icon at the top right corner
    - Then save it as `PDF` format.

## License

This tutorial is based off <https://github.com/esp-rs/no_std-training>.

- The code under `exercises/` licensed under the [MIT License][mit].
- The text in the book is under of the [CC-BY-SA V4][cc] license.

### Contribution

Code contributions are under MIT license.
Any book-text contributions are under CC-BY-SA v4.0 license.

[mit]: ./LICENSE-MIT
[cc]: ./LICENSE-CC-BY-SA
