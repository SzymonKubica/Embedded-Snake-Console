#![no_std]
#![no_main]

extern crate arduino_hal;
extern crate arrayvec;

mod analog_stick;
mod controller;
mod concurrency;
mod game_engine;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use arrayvec::ArrayVec;

use crate::analog_stick::AnalogStick;
use crate::concurrency::{Scheduler, SchedulerTask};
use crate::controller::Direction;
use crate::game_engine::GameEngine;

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

        let mut engine: GameEngine = GameEngine::new();

        let stick : AnalogStick = AnalogStick::new(
            x_pin,
            y_pin,
            switch_pin,
            &engine);


        let mut tasks: ArrayVec<&dyn SchedulerTask, 10> = ArrayVec::new();
        tasks.push(&engine as &dyn SchedulerTask);
        tasks.push(&stick as &dyn SchedulerTask);

        let scheduler: Scheduler = Scheduler::new(tasks, 25);
        scheduler.run();
    }
}


