use arduino_hal::hal::port::{PC0, PC1, PC2};
use arduino_hal::port::{mode::Analog, Pin};
use crate::controller::Controller;
use crate::controller::Direction;

pub struct AnalogStick {
    x_pin: Pin<Analog, PC0>,
    y_pin: Pin<Analog, PC1>,
    switch_pin: Pin<Analog, PC2>,

    x_value: i32,
    y_vaue: i32,
    switch_value: i32,
}


impl AnalogStick {
    pub fn new(
        x_pin: Pin<Analog, PC0>,
        y_pin: Pin<Analog, PC1>,
        switch_pin: Pin<Analog, PC2>) -> AnalogStick
    {
        AnalogStick {
            x_pin,
            y_pin,
            switch_pin,
            x_value: 0,
            y_vaue: 0,
            switch_value: 0
        }
    }
}

impl Controller for AnalogStick {
    fn get_direction(&self) -> Direction {
        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let mut adc = arduino_hal::Adc::new(peripherals.ADC, Default::default());
        self.x_pin.analog_read(&mut adc);

        Direction::UP
    }

}
