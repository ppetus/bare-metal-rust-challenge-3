Bare metal rust challenge 3 submission
==============================
The program targets a ARM platform using QEMU simulator on a linux machine.

Running "cargo build" will compile the program and the dependencies for the target "thumbv7m-none-eabi".
Running "cargo run" will run the program in the qemu-system-arm simulator as shown in the .cargo/config.toml configuration file.

The program initially sets up a cortex-m timer to interrupt the program every 10ms after which it will enter sleep mode. The interrupt handler will record the number of occurred interrupt to a global variable. Once 10 interrupts have occurred, the main program will disable the interrupt, add the timer reload value to itself and load that as the new reload value (double the reload value), reset the global variable, enable interrupts again and go back to sleep. This repeats in a never ending loop.

The program is written to function with only the cortex-m related crates as device specific hal crates did not work in QEMU.
