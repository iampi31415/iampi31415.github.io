# Log lines, rarely needed.
# Read the manual before uncommenting.
# set logging redirect on
# set logging file ./gdb.log
# set logging overwrite on
# set logging enabled on

# The binary file from dev profile.
file target/riscv32imac-unknown-none-elf/debug/gdb_hello_world
# default port opened by openocd.
target extended-remote localhost:3333
monitor reset halt
break gdb_hello_world::__risc_v_rt__main
# add a breakpoint in the a/=2 line
layout src
continue
