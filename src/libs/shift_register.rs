//! Serial-in parallel-out shift register

use core::cell::RefCell;
use core::ptr;

use embedded_hal::digital::v2::OutputPin;

trait ShiftRegisterInternal {
    fn update(&self, index: usize, command: bool);
}

/// Output pin of the shift register
pub struct ShiftRegisterPin<'a>
{
    shift_register: &'a dyn ShiftRegisterInternal,
    index: usize,
}

impl<'a> ShiftRegisterPin<'a>
{
    fn new(shift_register: &'a dyn ShiftRegisterInternal, index: usize) -> Self {
        ShiftRegisterPin { shift_register, index }
    }
}

impl<'a> OutputPin for ShiftRegisterPin<'a>
{
    fn set_low(&mut self) -> Result<(), ()> {
        Ok(self.shift_register.update(self.index, false))
    }

    fn set_high(&mut self) -> Result<(), ()> {
        Ok(self.shift_register.update(self.index, true))
    }

    type Error = ();
}

macro_rules! ShiftRegisterBuilder {
    ($name: ident, $size: expr) => {
        /// Serial-in parallel-out shift register
        pub struct $name<Pin1, Pin2, Pin3, Pin4>
            where Pin1: OutputPin,
                  Pin2: OutputPin,
                  Pin3: OutputPin,
                  Pin4: OutputPin,
        {
            clock: RefCell<Pin1>,
            latch: RefCell<Pin2>,
            data: RefCell<Pin3>,
            pin_0_supply: RefCell<Pin4>, // Need an external voltage supply as output 0 of my
                                            // shift register doesn't give enough voltage.
            output_state: RefCell<[bool; $size]>,
        }

        impl<Pin1, Pin2, Pin3, Pin4> ShiftRegisterInternal for $name<Pin1, Pin2, Pin3, Pin4>
            where Pin1: OutputPin,
                  Pin2: OutputPin,
                  Pin3: OutputPin,
                  Pin4: OutputPin,
        {
            /// Sets the value of the shift register output at `index` to value `command`
            fn update(&self, index: usize, command: bool) {
                self.output_state.borrow_mut()[index] = command;
                let output_state = self.output_state.borrow();
                self.latch.borrow_mut().set_low();

                for i in 1..=output_state.len() {
                    if output_state[output_state.len()-i] {
                        self.set_pin(output_state.len()-i, true)
                    } else {
                        self.set_pin(output_state.len()-i, false)
                    }
                    self.clock.borrow_mut().set_high();
                    self.clock.borrow_mut().set_low();
                }

                self.latch.borrow_mut().set_high();
            }
        }


        impl<Pin1, Pin2, Pin3, Pin4> $name<Pin1, Pin2, Pin3, Pin4>
            where Pin1: OutputPin,
                  Pin2: OutputPin,
                  Pin3: OutputPin,
                  Pin4: OutputPin,
        {
            /// Creates a new SIPO shift register from clock, latch, and data output pins
            pub fn new(clock: Pin1, latch: Pin2, data: Pin3, ground_0: Pin4) -> Self {
                $name {
                    clock: RefCell::new(clock),
                    latch: RefCell::new(latch),
                    data: RefCell::new(data),
                    pin_0_supply: RefCell::new(ground_0),
                    output_state: RefCell::new([false; $size]),
                }
            }

            /// Sets the voltage of a given output respectively, if requested to
            /// set the output_0 to high, it instead sets the pin_0_supply to
            /// high, because output_0 doesn't supply enough voltage.
            fn set_pin(&self, index: usize, set_high: bool) {
                if index == 0 {
                    if set_high { self.pin_0_supply.borrow_mut().set_high(); }
                    else { self.pin_0_supply.borrow_mut().set_low(); }
                }

                if set_high { self.data.borrow_mut().set_high(); }
                else { self.data.borrow_mut().set_low(); }
            }

            /// Get embedded-hal output pins to control the shift register outputs
            pub fn decompose(&self) -> [ShiftRegisterPin; $size] {
                let mut pins: [ShiftRegisterPin; $size];

                unsafe {
                    pins = core::mem::uninitialized();
                    for (index, elem) in pins[..].iter_mut().enumerate() {
                        ptr::write(elem, ShiftRegisterPin::new(self, index));
                    }
                }

                pins
            }

            /// Consume the shift register and return the original clock, latch, and data output pins
            pub fn release(self) -> (Pin1, Pin2, Pin3, Pin4) {
                let Self{clock, latch, data, pin_0_supply, output_state: _} = self;
                (clock.into_inner(), latch.into_inner(), data.into_inner(), pin_0_supply.into_inner())
            }
        }
    }
}

ShiftRegisterBuilder!(ShiftRegister8, 8);

/// 8 output serial-in parallel-out shift register
pub type ShiftRegister<Pin1, Pin2, Pin3, Pin4> = ShiftRegister8<Pin1, Pin2, Pin3, Pin4>;
