#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::{entry, exception};
use cortex_m::Peripherals;
use cortex_m::peripheral::{SYST, DWT, DCB};
use cortex_m_semihosting::{debug, hprintln};
use cortex_m::asm::wfi;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    
    // Set up the timer to interrupt every 10ms
    let mut counter = peripherals.SYST;
    counter.set_reload(SYST::get_ticks_per_10ms());
    counter.clear_current();
    counter.enable_interrupt();
    counter.enable_counter();
 
    // Setting up the system control block
    let mut scb = peripherals.SCB;
    scb.set_sleepdeep();
    hprintln!("Wait for interrupt");
    wfi();  // Sleep

    // Enable debugger trace
    let mut dcb = peripherals.DCB;
    dcb.enable_trace();

    // Get dwt
    let mut dwt = peripherals.DWT;
    DWT::unlock();
    dwt.enable_cycle_counter();
    hprintln!("Cycle counter enabled: {}", DWT::cycle_counter_enabled());
    
    


    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[exception]
fn SysTick() {
    hprintln!("Interrupt happened");
    hprintln!("");
}