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

mod mvc;
mod common;
mod time_util;
mod game_engine;
mod matrix_view;
mod analog_stick;
#[allow(warnings, unused)]
mod shift_register;
mod internal_representation;

use mvc::View;

use crate::mvc::Task;
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

        let mut view: GameView = GameView::new();
        let mut engine: GameEngine = GameEngine::new(&mut view);
        let mut stick : AnalogStick = AnalogStick::new(&mut engine);

        let mut test_game_board = game_engine::initialize_board();

        test_game_board[3][3] = common::SNAKE_SEGMENT;

        view.update(test_game_board);

        loop {
            view.run();
        }
    }
}


