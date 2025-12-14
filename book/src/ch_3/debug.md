1. Access `exercises/gdb_hello_world`
2. Inspect the configuration files `openocd.cfg` (for `openocd` ) and `.gdbinit` (for `gdb`).
3. Execute `cargo run`, leave out `--release` to use the development profile.
4. Run `gdb` in one terminal window, and `openocd` in another window (same directory!).
5. The image shows what `gdb` should look like at this stage.
    - Should have a breakpoint in `_peripherals` which means `gdb` stopped execution there.
    <div class="center w420">
    <a href="/assets/gdb.png">
    <img src="/assets/gdb.png" alt="GDB screenshot showing our main.rs source code. It has a B where the breakpoint is."/>
    </a>
    </div>

6. Exit with `Ctrl+D` or `Ctrl+C`.

In the next chapter we will learn a few commands.
