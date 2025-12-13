# GDB exercise

## OpenOCD
Useful commands for `openocd`:

- `-f <cfg_file>`.
- `-d<n>`: 0-3 for off-verbose, respectively.
- `-l out.log`: puts the logs in that file.
- `-s`: directory to search for config files.

An example `openocd.cfg` file is:
```openocd
# Opens GDB port at 3333
# debug_level 1
# log_output ./openocd.log
gdb breakpoint_override hard
set ESP_RTOS hwthread
source [find board/esp32c6-builtin.cfg]
```

Start it as `openocd`, and will take the config file.

## GDB 
An example `.gdbinit`:

```gdb
# default port opened by openocd.
target extended-remote localhost:3333
monitor reset halt
#maintenance flush register-cache
b main
layout src
c
```

Start it as `gdb -f <elf_path>`

Shortcuts:

- `C-b` Move back one character.
- `C-f` Move forward one character.
- `DEL` or `Backspace` Delete the character to the left of the cursor.
- `C-d` Delete the character underneath the cursor.
- `C-x C-u` Undo the last editing command.
- `C-a` Move to the start of the line.
- `C-e` Move to the end of the line.
- `M-f` Move forward a word, where a word is composed of letters and digits.
- `M-b` Move backward a word.
- `C-l` Clear the screen, reprinting the current line at the top
- `C-r` search backwards, `C-s` search forwards in history.
