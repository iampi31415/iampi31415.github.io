# Contributing

Please note that any contribution is under MIT license unless stated otherwise.

1. Fork, 
2. Create a new branch "fix-x"
3. Send PR.

## Best practices

These are a guideline, not mandatory. 

### No AI, no emojis

Please avoid:

- AI-written content.
- Emojis.

### Numbering

- Book chapters (folders) are numbered.
- Files are not numbered.
- The crates (`hello_world`, `panic` etc.) are not numbered.

This makes easier to adapt or change things as the book evolves.

### Links

- Instead of `[Some Text](https://some.link)` use `[Some Text]`. And at the bottom of the page add `[Some Text]: https://some.link`.
- This makes it easier to read the text.
- If you find trouble with some case, just use the `[..](..)` syntax, it's not a big deal.

### Model

- `esp32-c6`, no uppercases.

### Images

Including images, you may want to use the snippet:

```html
<div class="center w320"> <!--other classes: w220, w420-->
<!--Start with `/` following path from `src`.-->
    <a href="/assets/<image_name>.jpg">
        <img alt="Add Alt Description" src="/assets/<image_name>.jpg"/>
    </a>
    <p>Optional footer</p>
</div>
```
