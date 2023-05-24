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
   6. https://os.phil-opp.com/double-fault-exceptions/
   7. https://os.phil-opp.com/hardware-interrupts/
   8. https://os.phil-opp.com/paging-introduction/
2. https://wiki.osdev.org/
   1. https://wiki.osdev.org/Exceptions
   2. https://wiki.osdev.org/System_V_ABI
   3. https://wiki.osdev.org/Triple_Fault
   4. https://wiki.osdev.org/Double_Fault#Double_Fault
   5. https://wiki.osdev.org/8259_PIC
3. [https://pages.cs.wisc.edu/~remzi/OSTEP/vm-beyondphys.pdf](Beyond Physical Memory: Mechanisms)
4. [https://www.amd.com/system/files/TechDocs/24593.pdf](AMD64 Architecture Programmer’s Manual Volume 2: System Programming)
5. [http://pages.cs.wisc.edu/~remzi/OSTEP/](Operating Systems: Three Easy Pieces)
6. https://eli.thegreenplace.net/2011/01/27/how-debuggers-work-part-2-breakpoints
7. https://0xax.gitbooks.io/linux-insides/
8. https://github.com/torvalds/linux
9. https://linux-kernel-labs.github.io/refs/heads/master/lectures/intro.html
10. https://en.wikipedia.org/wiki/A20_line
11. https://en.wikipedia.org/wiki/Real_mode
12. https://en.wikipedia.org/wiki/I386
13. https://en.wikipedia.org/wiki/Power_good_signal
14. https://doc.rust-lang.org/1.30.0/book/first-edition/functions.html#diverging-functions
15. https://www.bogotobogo.com/cplusplus/stackunwinding.php
16. https://en.wikipedia.org/wiki/VGA-compatible_text_mode
    1. https://en.wikipedia.org/wiki/VGA_text_mode
