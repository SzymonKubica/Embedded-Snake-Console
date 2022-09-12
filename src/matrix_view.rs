use arduino_hal::port::Pin;
use arduino_hal::port::mode::Output;
use embedded_hal::digital::v2::OutputPin;
use arduino_hal::hal::port::{PB0, PB1, PB2, PB3, PB4};
use arduino_hal::hal::port::{PD2, PD3, PD4, PD5, PD6, PD7};
use crate::libs::shift_register::ShiftRegister;
use crate::mvc::{View, Task};

pub const SCREEN_REFRESH_INTERVAL: u32 = 150; // 150 microseconds.

pub struct GameView {
    screen: [[u8; 8]; 8],
    shift_register: ShiftRegister<
        Pin<Output, PB2>,
        Pin<Output, PB3>,
        Pin<Output, PB4>>,
    ground_pins: GroundPins
}

impl GameView {
    pub fn new(
        shift_register: ShiftRegister<
            Pin<Output, PB2>,
            Pin<Output, PB3>,
            Pin<Output, PB4>>,
        ground_pins: GroundPins) -> GameView {

        GameView {
            screen: Default::default(),
            shift_register,
            ground_pins,
        }
    }
}

impl View for GameView {
    fn update(&mut self, game_board: [[u8; 8]; 8]) -> () {
        self.screen = game_board;
    }

}

impl Task for GameView {
    fn run(&mut self) -> () {
        let mut outputs = self.shift_register.decompose();
        for i in 0..8_usize {
            outputs[i].set_high().ok(); // Add voltage to the ith row of the matrix


            // Set the corresponding ground pin to low to complete the circuit
            // and make the led ligth up.
            for j in 0..8_usize {
                let current_pixel = self.screen[j][i];
                if current_pixel != 0 {
                   self.ground_pins.set_pin_low(j)
                }
                arduino_hal::delay_us(SCREEN_REFRESH_INTERVAL);
                self.ground_pins.disconnect_ground();
            }
            outputs[i].set_low().ok();
        }
    }
}


pub struct GroundPins {
    ground_0: Pin<Output, PD2>,
    ground_1: Pin<Output, PD3>,
    ground_2: Pin<Output, PD4>,
    ground_3: Pin<Output, PD5>,
    ground_4: Pin<Output, PD6>,
    ground_5: Pin<Output, PD7>,
    ground_6: Pin<Output, PB0>,
    ground_7: Pin<Output, PB1>
}

impl GroundPins {
    pub fn new(
        ground_0: Pin<Output, PD2>,
        ground_1: Pin<Output, PD3>,
        ground_2: Pin<Output, PD4>,
        ground_3: Pin<Output, PD5>,
        ground_4: Pin<Output, PD6>,
        ground_5: Pin<Output, PD7>,
        ground_6: Pin<Output, PB0>,
        ground_7: Pin<Output, PB1>
    ) -> GroundPins {

        GroundPins {
            ground_0,
            ground_1,
            ground_2,
            ground_3,
            ground_4,
            ground_5,
            ground_6,
            ground_7
        }
    }

    pub fn set_pin_low(&mut self, index: usize) {
        match index {
            0 => self.ground_0.set_low(),
            1 => self.ground_1.set_low(),
            2 => self.ground_2.set_low(),
            3 => self.ground_3.set_low(),
            4 => self.ground_4.set_low(),
            5 => self.ground_5.set_low(),
            6 => self.ground_6.set_low(),
            7 => self.ground_7.set_low(),
            _ => ()
        }
    }

    fn disconnect_ground(&mut self) {
        for i in 0..8usize {
            self.set_pin_high(i);
        }
    }

    fn set_pin_high(&mut self, index: usize) {
        match index {
            0 => self.ground_0.set_high(),
            1 => self.ground_1.set_high(),
            2 => self.ground_2.set_high(),
            3 => self.ground_3.set_high(),
            4 => self.ground_4.set_high(),
            5 => self.ground_5.set_high(),
            6 => self.ground_6.set_high(),
            7 => self.ground_7.set_high(),
            _ => ()
        }
    }

}
