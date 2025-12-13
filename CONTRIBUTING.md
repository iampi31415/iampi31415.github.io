# Contributing

Please note that any contribution is under MIT license unless stated otherwise.

The `README.md` explains the process of fork, clone, PR.

Here, we explain just best writing practices. If you don't follow them it's not an issue, it's just to keep it consistent.

The book was written in British English, but it is not mandatory.

## No AI, no emojis

Please avoid:

- Sendind AI-written content.
- Use of emojis in the content.

## Numbering

- Book chapters are numbered, but not files.
- The crates (`hello_world`, `panic` etc.) are not numbered.

This makes easier to adapt or change things as the book evolves.

## Links

- Instead of `[Some Text](https://some.link)` use `[Some Text]`. And at the bottom of the page add `[Some Text]: https://some.link`.
- This makes it easier to read the text.
- If you find trouble with some case, just use the `[..](..)` syntax, it's not a big deal.

## Board and chip

- Use simply `esp32-c6` board,
- Use `esp32-c6` chip or MCU.

## Images

Including images, you may want to use the snippet:

```html
<div class="center w320"> <!--There are other classes like w220, w420-->
<a href="../assets/<image_name>.jpg">
<img alt="Add Alt Description" src="../assets/<image_name>.jpg"/>
</a>
<p>Optional footer</p>
</div>
```
