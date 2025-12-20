### GDB commands

- Program Flow:
    - `next` step one line over.
    - `continue` or `c`: continue executing up to next break point.
    - `break fn_name`: break point at function name.
        - There may be many with same name. So we need the path.
            - For example: `break gdb_hello_world::__risc_v_rt__main`
    - `break lineno`: break point at line number of currently-focused file.
        - Example: `break 19`, or `br 19`.
    - `break my_file.rs:lineno`: example `break main.rs:17`.
    - `monitor reset halt`: restarts and halts it.
    - `load`: if we only use `cargo build`, then we can `flash` using `load`.
        - Within `gdb` type `load`. It'll flash the file to the MCU.
- Layout:
    - `layout src`: shows source code and CLI.
    - `layout asm`: shows assembly source and CLI.
    - `tui disable` to disable the layout.
- Overviews:
    - `info break` to show breakpoints
    - `info locals` to show variables.
    - `print x` to print variable `x`. Also `print &x` prints the address of `x`.
    - `list` or `list main`: will show our program with numbered lines.

### Exercises

1. Access `exercises/gdb_hello_world`
2. Inspect the configuration files `openocd.cfg` (for `openocd` ) and `.gdbinit` (for `gdb`).
3. Execute `cargo run`, leave out `--release` to use the development profile.
    - Without it, cargo will remove / optimise lines.
    - We _should_ debug with `--release` to ensure the best outcomes.
    - But this examples is just to get started.
4. Run `gdb -x gdbinit -q` in one terminal, and `openocd` in another (at the same directory!).
    - In the image, `_peripherals` has not yet been assigned. `gdb` stopped execution there.
      The window at the bottom half is our `(gdb)` prompt.
      <div class="center w420">
      <a href="/assets/gdb.png">
      <img src="/assets/gdb.png" alt="GDB screenshot showing our main.rs source code. It has a B where the breakpoint is."/>
      </a>
      </div>
5. Add a `break` point where `a/=2`, then `print` the resulting value.
6. Exit with `Ctrl+D` or `Ctrl+C`.

### Suggested Reading

- [MB2] article on Debugging.

[MB2]: https://docs.rust-embedded.org/discovery-mb2/05-meet-your-software/debug-it.html
