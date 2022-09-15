#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(trait_upcasting)]
#![allow(incomplete_features)]
#![feature(asm_experimental_arch)]

use core::panic::PanicInfo;

extern crate arduino_hal;
extern crate avr_device;
extern crate embedded_hal;
extern crate arrayvec;
extern crate oorandom;

mod mvc;
mod common;
mod game_engine;
mod matrix_view;
mod analog_stick;
mod user_interface;
mod internal_representation;
mod libs;

use common::CONTROLLER_POLLING_INTERVAL;
use internal_representation::controller_input::ControllerInput;
use libs::time_util::millis_init;
use matrix_view::GroundPins;
use mvc::{TimedRunnable, Controller, Model};
use libs::shift_register::ShiftRegister;

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

        // Enable the millis() function
        millis_init(peripherals.TC0);
        unsafe { avr_device::interrupt::enable() }

        // Initialise the view.
        let clock_pin = pins.d10.into_output();
        let latch_pin = pins.d11.into_output();
        let data_pin = pins.d12.into_output();
        let ground_0_supply = pins.d13.into_output();

        let ground_pins = GroundPins::new(
            pins.d2.into_output_high(),
            pins.d3.into_output_high(),
            pins.d4.into_output_high(),
            pins.d5.into_output_high(),
            pins.d6.into_output_high(),
            pins.d7.into_output_high(),
            pins.d8.into_output_high(),
            pins.d9.into_output_high());

        let shift_register = ShiftRegister::new(
            clock_pin,
            latch_pin,
            data_pin,
            ground_0_supply);

        let mut view = GameView::new(shift_register, ground_pins);

        let mut ad_converter = arduino_hal::Adc::new(
            peripherals.ADC, Default::default());

        let random_noise_pin = pins.a3.into_analog_input(&mut ad_converter);
        let random_seed = random_noise_pin.analog_read(&mut ad_converter);
        // Initialise the engine.
        let mut engine = GameEngine::new(&mut view, random_seed);

        // Initialise the controller.
        let x_pin = pins.a0.into_analog_input(&mut ad_converter);
        let y_pin = pins.a1.into_analog_input(&mut ad_converter);
        let switch_pin = pins.a2.into_pull_up_input();

        let mut stick = AnalogStick::new(
            x_pin,
            y_pin,
            switch_pin,
            ad_converter);

        loop {
            let input: ControllerInput = stick.read_input();
            engine.on_input(input);
            engine.run_for(CONTROLLER_POLLING_INTERVAL);
        }
    }
}


