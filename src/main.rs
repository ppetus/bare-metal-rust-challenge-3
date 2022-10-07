#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::Peripherals;
use cortex_m::peripheral::SYST;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    
    // Set up the timer
    let mut counter = peripherals.SYST;
    counter.set_reload(0xff_ffff);
    counter.clear_current();
    counter.enable_counter();
 
    // Print config info about the timer
    hprintln!("Is counter   enabled: {}",   counter.is_counter_enabled());
    hprintln!("Is interrupt enabled: {}",   counter.is_interrupt_enabled());
    hprintln!("Clk source          : {:?}", counter.get_clock_source());
    hprintln!("Is precise          : {}", SYST::is_precise());
    hprintln!("Has ref clock       : {}", SYST::has_reference_clock());

    // Print clock info
    hprintln!("Wrapped? {}", counter.has_wrapped());
    hprintln!("Current? {:x}", SYST::get_current());

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
