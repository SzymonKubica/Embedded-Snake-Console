use arduino_hal::hal::port::{PC0, PC1, PC2};
use arduino_hal::port::{mode::Analog, Pin};
use crate::controller::Controller;
use crate::controller::Direction;

pub struct AnalogStick {
    x_pin: Pin<Analog, PC0>,
    y_pin: Pin<Analog, PC1>,
    switch_pin: Pin<Analog, PC2>,
}


impl AnalogStick {
    pub fn new(
        x_pin: Pin<Analog, PC0>,
        y_pin: Pin<Analog, PC1>,
        switch_pin: Pin<Analog, PC2>) -> AnalogStick
    {
        AnalogStick { x_pin, y_pin, switch_pin }
    }
}

impl Controller for AnalogStick {
    fn get_direction(&self) -> Direction {
        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let mut adc = arduino_hal::Adc::new(peripherals.ADC, Default::default());
        let x_value: u16 = self.x_pin.analog_read(&mut adc);
        let y_value: u16 = self.y_pin.analog_read(&mut adc);

        if x_value < 200 {
            return Direction::UP;
        }
        if y_value < 200 {
            return Direction::RIGHT;
        }
        if x_value > 800 {
            return Direction::DOWN;
        }
        if y_value > 800 {
            return Direction::LEFT;
        }
        return Direction::NO_DIRECTION;
    }
}
