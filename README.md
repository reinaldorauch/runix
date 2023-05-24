# runix

rust implementation of linux kernel poc

This is a proof of concept that aims to implement the internals of the kernel but maintaining compatibility with the ecosystem around.

to keep it simple, this will focus on intel/amd x64 instruction set.

I don't hope to this get any attention but here it goes anyway

## Changelog

1. Right now (may, 2023) I'm following the great Philipp Oppermann' series (see first Reference) of building a kernel from scratch with rust.

## References

1. https://os.phil-opp.com/
   1. https://os.phil-opp.com/freestanding-rust-binary/
   2. https://os.phil-opp.com/minimal-rust-kernel/
   3. https://os.phil-opp.com/vga-text-mode/
   4. https://os.phil-opp.com/testing/
   5. https://os.phil-opp.com/cpu-exceptions/
2. https://wiki.osdev.org/Exceptions
   1. https://wiki.osdev.org/System_V_ABI
   2. https://wiki.osdev.org/Triple_Fault
   3. https://wiki.osdev.org/Double_Fault#Double_Fault
3. https://eli.thegreenplace.net/2011/01/27/how-debuggers-work-part-2-breakpoints
4. https://0xax.gitbooks.io/linux-insides/
5. https://github.com/torvalds/linux
6. https://linux-kernel-labs.github.io/refs/heads/master/lectures/intro.html
7. https://en.wikipedia.org/wiki/A20_line
8. https://en.wikipedia.org/wiki/Real_mode
9. https://en.wikipedia.org/wiki/I386
10. https://en.wikipedia.org/wiki/Power_good_signal
11. https://doc.rust-lang.org/1.30.0/book/first-edition/functions.html#diverging-functions
12. https://www.bogotobogo.com/cplusplus/stackunwinding.php
13. https://en.wikipedia.org/wiki/VGA-compatible_text_mode
    1. https://en.wikipedia.org/wiki/VGA_text_mode
