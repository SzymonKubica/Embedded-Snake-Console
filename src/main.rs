#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    loop {
        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(peripherals);


        let mut clock_pin = pins.d10.into_output();
        let mut latch_pin = pins.d11.into_output();
        let mut data_pin = pins.d12.into_output();

        let ground_pins = (
            pins.d2.into_output_high(),
            pins.d3.into_output_high(),
            pins.d4.into_output_high(),
            pins.d5.into_output_high(),
            pins.d6.into_output_high(),
            pins.d7.into_output_high(),
            pins.d8.into_output_high(),
            pins.d9.into_output_high(),
        );

        const DELAY_INTERVAL: i32 = 100;
        const FRAME_MOVE_THRESHOLD: i32 = 25;
        let mut frame_counter = 0;

        let x_pin = pins.a0.into_analog_input();


        loop {

        }
    }
}


