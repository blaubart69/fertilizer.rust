use std::error::Error;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rppal::gpio::{Gpio, InputPin, Level, Trigger};

use crate::ring_buffer::RingBuffer;

pub struct GpioSignals{
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

    pub fn start(&mut self, signals_wheel : Arc<Mutex<RingBuffer>>, signals_roller  : Arc<Mutex<RingBuffer>>) -> Result<(), Box<dyn Error>> {
        self.pin_wheel .set_async_interrupt(Trigger::FallingEdge,move |_level| { signals_wheel .lock().unwrap().push(Instant::now()) })?;
        self.pin_roller.set_async_interrupt(Trigger::FallingEdge,move |_level| { signals_roller.lock().unwrap().push(Instant::now()) })?;
        Ok(())
    }
}
