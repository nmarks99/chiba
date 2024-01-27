#![no_std]
#![no_main]

use rp_pico::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_halt as _;
use rp_pico::hal::pac;
use rp_pico::hal;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    
    // Single-cylce I/O block
    let sio = hal::Sio::new(pac.SIO);
    
    // Set the pins according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS
    );

    // LED output, button input
    let mut led_pin = pins.led.into_push_pull_output();
    let button_pin = pins.gpio17.into_pull_up_input();

    // set led if button is pushed
    loop {
        if button_pin.is_low().unwrap() {
            led_pin.set_high().unwrap();
        }
        else {
            led_pin.set_low().unwrap();
        }
    }
}
