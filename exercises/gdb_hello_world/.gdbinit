#set remotetimeout 10
# default port opened by open ocd.
target extended-remote localhost:3333
monitor reset halt
#maintenance flush register-cache
b main
layout src
c
