#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(trait_upcasting)]
#![allow(incomplete_features)]

extern crate arduino_hal;
extern crate avr_device;
extern crate embedded_hal;
extern crate arrayvec;

mod analog_stick;
mod mvc;
mod game_engine;
mod matrix_view;
mod time_util;

use core::panic::PanicInfo;

use crate::analog_stick::AnalogStick;
use crate::game_engine::GameEngine;
use crate::matrix_view::GameView;

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

        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let mut adc = arduino_hal::Adc::new(peripherals.ADC, Default::default());

        let x_pin = pins.a0.into_analog_input(&mut adc);
        let y_pin = pins.a1.into_analog_input(&mut adc);
        let switch_pin = pins.a2.into_analog_input(&mut adc);

        let mut x_val: i32 = 0;
        let mut y_val: i32 = 0;
        let mut s_val: i32 = 0;

        let mut view: GameView = GameView {  };


        let mut engine: GameEngine = GameEngine::new(&mut view);

        // Initialise the analog stick and let the scheduler set it.
        let mut stick : AnalogStick = AnalogStick::new(
            x_pin,
            y_pin,
            switch_pin,
            &mut adc,
            &mut engine);

    }
}


