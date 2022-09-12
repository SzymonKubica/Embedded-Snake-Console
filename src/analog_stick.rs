use arduino_hal::Adc;
use arduino_hal::port::mode::{Input, PullUp};
use arduino_hal::hal::port::{PC0, PC1, PC2};
use arduino_hal::port::{mode::Analog, Pin};

use crate::mvc::{Model, Controller, Direction, ControllerInput};

const ANALOG_LOWER_THRESHOLD: u16 = 200;
const ANALOG_UPPER_THRESHOLD: u16 = 800;

pub struct AnalogStick<'a> {
    x_pin: Pin<Analog, PC0>,
    y_pin: Pin<Analog, PC1>,
    switch_pin: Pin<Input<PullUp>, PC2>,
    ad_converter: Adc,
    listener: &'a mut dyn Model,
}

impl<'a> AnalogStick<'a> {
    pub fn new(
            x_pin: Pin<Analog, PC0>,
            y_pin: Pin<Analog, PC1>,
            switch_pin: Pin<Input<PullUp>, PC2>,
            ad_converter: Adc,
            listener: &'a mut dyn Model) -> AnalogStick<'a> {

        AnalogStick {
            x_pin,
            y_pin,
            switch_pin,
            ad_converter,
            listener }
    }
}

impl<'a> Controller<'a> for AnalogStick<'a> {
    /*
     * The values read from the analog stick range from 0 to 1024 with some
     * minor fluctuations caused by hardware deficiencies. In the neutral
     * position, the two values read from the analog pins of the stick should be
     * roughly equal to 500. Then a value of 0 on the x_pin would indicate that
     * the stick has been fully displaced up, whereas a value of 1024 would indicate
     * that it was fully moved down. The threshold when we detect a choice of
     * direction is a bit lower to account for the inaccuracies of the measurement
     * At present the lower threshold is 200 whereas the upper one is 800.
     *
     */
    fn read_input(&mut self) -> ControllerInput {
        let x_value: u16 = self.x_pin.analog_read(&mut self.ad_converter);
        let y_value: u16 = self.y_pin.analog_read(&mut self.ad_converter);

        let mut direction: Direction = Direction::NoDirection;

        if x_value < ANALOG_LOWER_THRESHOLD {
             direction = Direction::Up;
        }
        if x_value > ANALOG_UPPER_THRESHOLD {
            direction = Direction::Down;
        }
        if y_value < ANALOG_LOWER_THRESHOLD {
            direction = Direction::Right;
        }
        if y_value > ANALOG_UPPER_THRESHOLD {
            direction = Direction::Left;
        }

        let switch_input: bool = self.switch_pin.is_low();

        ControllerInput::new(switch_input, direction)
    }

    fn notify_listener(&mut self, input: ControllerInput) -> () {
        self.listener.on_input(input);
    }
}
