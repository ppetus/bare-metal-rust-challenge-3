#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::{entry, exception};
use cortex_m::Peripherals;
use cortex_m::peripheral::SYST;
use cortex_m_semihosting::{debug, hprintln};
use cortex_m::asm::wfi;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    
    // Set up the timer
    let mut counter = peripherals.SYST;
    counter.set_reload(0x0f_ffff);
    counter.clear_current();
    counter.enable_interrupt();
    counter.enable_counter();
 
    // Print config info about the timer
    hprintln!("Is counter   enabled: {}",   counter.is_counter_enabled());
    hprintln!("Is interrupt enabled: {}",   counter.is_interrupt_enabled());
    hprintln!("Clk source          : {:?}", counter.get_clock_source());
    hprintln!("Is precise          : {}", SYST::is_precise());
    hprintln!("Has ref clock       : {}", SYST::has_reference_clock());
    hprintln!("");

    // Print clock info
    hprintln!("Wrapped? {}", counter.has_wrapped());
    hprintln!("Current? {:x}", SYST::get_current());
    hprintln!("");

    // Setting up the system control block
    let mut scb = peripherals.SCB;
    scb.set_sleepdeep();
    hprintln!("Wait for interrupt");
    wfi();  // Sleep

    // Print clock info
    hprintln!("Wrapped? {}", counter.has_wrapped());
    hprintln!("Current? {:x}", SYST::get_current());
    hprintln!("");


    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[exception]
fn SysTick() {
    hprintln!("Interrupt happened");
    hprintln!("");
}