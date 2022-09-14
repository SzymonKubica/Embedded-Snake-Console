use arduino_hal::Adc;
use arduino_hal::port::mode::{Input, PullUp};
use arduino_hal::hal::port::{PC0, PC1, PC2};
use arduino_hal::port::{mode::Analog, Pin};

use crate::mvc::Controller;

use crate::internal_representation::controller_input::ControllerInput;
use crate::internal_representation::direction::Direction;
use crate::internal_representation::pin_state::PinState;

const ANALOG_LOWER_THRESHOLD: u16 = 200;
const ANALOG_UPPER_THRESHOLD: u16 = 800;

pub struct AnalogStick {
    x_pin: Pin<Analog, PC0>,
    y_pin: Pin<Analog, PC1>,
    switch_pin: Pin<Input<PullUp>, PC2>,
    switch_state: PinState,
    ad_converter: Adc,
}

impl<'a> AnalogStick {
    pub fn new(
            x_pin: Pin<Analog, PC0>,
            y_pin: Pin<Analog, PC1>,
            switch_pin: Pin<Input<PullUp>, PC2>,
            ad_converter: Adc) -> AnalogStick {

        AnalogStick {
            x_pin,
            y_pin,
            switch_pin,
            switch_state: PinState::High,
            ad_converter }
    }
}

impl Controller for AnalogStick {
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
        let switch_input = PinState::from(self.switch_pin.is_high());

        let toggle_signal = toggle_registered(switch_input, self.switch_state);
        let direction = determine_direction(x_value, y_value);

        self.switch_state = switch_input;

        ControllerInput::new(toggle_signal, direction)
    }
}

fn toggle_registered(new_state: PinState, old_state: PinState) -> bool {
    old_state == PinState::High && new_state == PinState::Low
}

fn determine_direction(x_value: u16, y_value: u16) -> Direction {
    let mut direction: Direction = Direction::NoDirection;

    if x_value < ANALOG_LOWER_THRESHOLD { direction = Direction::Up; }
    if x_value > ANALOG_UPPER_THRESHOLD { direction = Direction::Down; }
    if y_value < ANALOG_LOWER_THRESHOLD { direction = Direction::Right; }
    if y_value > ANALOG_UPPER_THRESHOLD { direction = Direction::Left; }

    direction
}


