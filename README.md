# Rust Embedded Basics (esp32-c6)

Learn the basics of Embedded programming in Rust _with a esp32-c6 board_.

The website is at <https://iampi31415.github.io>.

The book has its source code under `book/`, the code-exercises under `exercises/`.

## Contributions

Please see [Contributing.md](./CONTRIBUTING.md).

## Exercises

Get the exercises:

- Clone the repo to use the exercises:

    ```bash
    git clone https://github.com/iampi31415/iampi31415.github.io
    ```

## Book

The book is at <https://iampi31415.github.io>. To download the book one of:

- Clone the `gh-pages` branch:

    ```bash
    git clone -b gh-pages git@github.com:iampi31415/iampi31415.github.io
    ```

    - Now you've a copy of the website
    - Click `index.hmtl` to open it.

- Go to the book URL <https://iampi31415.github.io>

    - Click "Print" icon at the top right corner
    - Then save it as `PDF` format.

- It can also be built locally with. This requires:
    1. Clone repo
        ```bash
        git clone https://github.com/iampi31415/iampi31415.github.io learn_embed
        ```
    2. Run `cargo install mdbook@0.5.1`
    3. Access `learn_embed/book/`.
    4. Then `mdbook serve --open` to build and serve.

## License

This tutorial is based off <https://github.com/esp-rs/no_std-training>, at the [time of this commit.]

- The code under `exercises/` licensed under the [MIT License][mit].
- The text in the book is under of the [CC-BY-SA V4][cc] license.

### Contribution

Code contributions are under MIT license.

Any book-prose contributions are under CC-BY-SA v4.0 license. The book's code snippets are also under MIT license.

[time of this commit]: https://github.com/esp-rs/no_std-training/commit/deb5f68b09daa5c14eba3f96d97b79c028409566
[mit]: ./LICENSE-MIT
[cc]: ./LICENSE-CC-BY-SA
