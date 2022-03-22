use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Instant;
use rppal::gpio::{Gpio, InputPin, Level, Trigger};
use SignalKind::WHEEL;
use crate::SignalKind;

struct GpioSignals{
    gpio : Gpio,
    pin_wheel : InputPin,
    pin_roller : InputPin,
}

impl GpioSignals {
    pub fn new() -> Result<GpioSignals, Box<dyn Error>> {
        let gpio = Gpio::new()?;
        let mut pin_wheel = gpio.get(23)?.into_input_pulldown();
        let mut pin_roller = gpio.get(24)?.into_input_pulldown();

        Ok(GpioSignals {
            gpio,
            pin_wheel,
            pin_roller,
        })
    }

    pub fn start<F>(&mut self, on_signal_wheel : F, on_signal_roller : F) -> Result<(), Box<dyn Error>>
        where F : FnMut(Level) + Send + 'static {

        self.pin_wheel.set_async_interrupt(Trigger::FallingEdge,on_signal_wheel)?;
        self.pin_roller.set_async_interrupt(Trigger::FallingEdge,on_signal_roller )?;

        Ok(())
    }
}
