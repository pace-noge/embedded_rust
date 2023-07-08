file target/thumbv7m-none-eabi/debug/embedded_rust
target remote :3333

set print asm-demangle on
set print pretty on

load

break main

continue