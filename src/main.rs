#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::{entry, exception};
use cortex_m::Peripherals;
use cortex_m::peripheral::SYST;
use cortex_m_semihosting::hprintln;
use cortex_m::asm::wfi;

static mut NUM_INTERRUPTS: i32 = 0;

#[entry]
fn main() -> ! {
   let peripherals = Peripherals::take().unwrap();
    
    // Set up the timer to interrupt every 10ms in the beginning
    let mut counter = peripherals.SYST;
    let mut counter_reload_value = SYST::get_ticks_per_10ms();
    counter.set_reload(counter_reload_value);
    counter.clear_current();
    counter.enable_interrupt();
    counter.enable_counter();
 
    // Setting up the system control block to set the device to deep sleep between interrupts
    let mut scb = peripherals.SCB;
    scb.set_sleepdeep();

    // Local variable to minimize the amount of lines inside unsafe
    let mut local_num_interrupts;
    loop {
        hprintln!("Going to sleep");
        wfi();
        unsafe { local_num_interrupts = NUM_INTERRUPTS; }
        
        // Increse the interval of interrupts after ten interrupts
        if local_num_interrupts >= 10 {
            counter.disable_interrupt();  // Do not want to get interrupted heres
            hprintln!("\n10 interrupts occurred. Raising the timer reload value and resetting the number of occurred interrupts\n");
            counter_reload_value += SYST::get_reload();  // Basically overly complicated way to double the value :)
            counter.set_reload(counter_reload_value);
            counter.clear_current();

            local_num_interrupts = 0;
            unsafe {NUM_INTERRUPTS = 0;}

            counter.enable_interrupt();  // Continue the program execution
        }
    }
}

// Interrupt handler to record the number of occurred timer interrupts
#[exception]
fn SysTick() {
    unsafe {
        NUM_INTERRUPTS += 1;
        hprintln!("New interrupt occurred! Number of occurred interrupts: {}", NUM_INTERRUPTS);
    }
}