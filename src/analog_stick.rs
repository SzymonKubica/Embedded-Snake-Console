use arduino_hal::hal::port::{PC0, PC1, PC2};
use arduino_hal::port::{mode::Analog, Pin};

use crate::concurrency::SchedulerTask;
use crate::controller::{Controller, ControllerListener};
use crate::controller::Direction;

pub struct AnalogStick<'a> {
    x_pin: Pin<Analog, PC0>,
    y_pin: Pin<Analog, PC1>,
    switch_pin: Pin<Analog, PC2>,
    listener: &'a dyn ControllerListener,
}

impl<'a> AnalogStick<'a> {
    pub fn new(
        x_pin: Pin<Analog, PC0>,
        y_pin: Pin<Analog, PC1>,
        switch_pin: Pin<Analog, PC2>,
        listener: &'a dyn ControllerListener) -> AnalogStick {

        AnalogStick { x_pin, y_pin, switch_pin, listener }
    }
}

impl<'a> Controller for AnalogStick<'a> {
    fn get_direction(&self) -> Direction {
        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let mut adc = arduino_hal::Adc::new(peripherals.ADC, Default::default());
        let x_value: u16 = self.x_pin.analog_read(&mut adc);
        let y_value: u16 = self.y_pin.analog_read(&mut adc);

        if x_value < 200 {
            return Direction::Up;
        }
        if y_value < 200 {
            return Direction::Right;
        }
        if x_value > 800 {
            return Direction::Down;
        }
        if y_value > 800 {
            return Direction::Left;
        }
        return Direction::NoDirection;
    }
}

impl<'a> SchedulerTask for AnalogStick<'a>  {
    fn run_task(&self, microseconds: i32) -> () {

    }

}
