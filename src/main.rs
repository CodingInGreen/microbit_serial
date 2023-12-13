#![no_main]
#![no_std]

use cortex_m_rt::entry;
//use core::mem::discriminant;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
//use panic_halt as _;
use microbit::{
    hal::pac,
    hal::gpio::{p0::Parts as P0Parts, Level},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("Program Starting!");

    let peripherals = pac::Peripherals::take().unwrap();

    // Initialize UART
    let uart0 = &peripherals.UART0;

    let gpio = P0Parts::new(peripherals.P0);

    // Configure GPIO pins
    let _tx_pin = gpio.p0_24.into_push_pull_output(Level::Low);
    let _rx_pin = gpio.p0_25.into_floating_input();


    // Configure UART
    uart0.psel.txd.write(|w| unsafe { w.bits(24) }); // TXD pin
    uart0.psel.rxd.write(|w| unsafe { w.bits(25) }); // RXD pin

    // Configure UART for transmission
    uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });

    // Set Baud Rate and enable write
    uart0.baudrate.write(|w| w.baudrate().baud115200());
    uart0.enable.write(|w| w.enable().enabled());

    // Send a string over UART
    write_uart0(uart0, "Hello World!\n").unwrap();

    // Stop UART transmission
    uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });

    loop {
        continue;
    }
}

fn write_uart0(uart0: &pac::UART0, s: &str) -> core::fmt::Result {
    uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });
        for c in s.as_bytes() {
            uart0.txd.write(|w| unsafe { w.bits(u32::from(*c)) });
    
            while uart0.events_txdrdy.read().bits() == 0 {}
    
            uart0.events_txdrdy.write(|w| unsafe { w.bits(0) });
        }
    uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
    Ok(())
}