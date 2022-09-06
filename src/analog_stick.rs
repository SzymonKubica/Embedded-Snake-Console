use arduino_hal::port::{mode::Analog, Pin};


pub struct AnalogStick<T1, T2, T3> {
    x_pin: Pin<Analog, T1>,
    y_pin: Pin<Analog, T2>,
    switch_pin: Pin<Analog, T3>,

    x_value: i32,
    y_vaue: i32,
    switch_value: i32,
}


impl<T1, T2, T3> AnalogStick<T1, T2, T3> {
    pub fn new(
        x_pin: Pin<Analog, T1>,
        y_pin: Pin<Analog, T2>,
        switch_pin: Pin<Analog, T3>) -> AnalogStick<T1, T2, T3>
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
