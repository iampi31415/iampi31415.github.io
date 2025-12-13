# default port opened by open ocd.
#set logging redirect on
set logging file ./gdb.log
set logging overwrite on
set logging enabled on
file target/riscv32imac-unknown-none-elf/release/gdb_hello_world
target extended-remote localhost:3333
monitor reset halt
br __risc_v_rt__main
layout src
c
