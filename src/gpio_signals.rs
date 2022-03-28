use std::error::Error;
use std::ops::{Add, Deref};
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::time::Instant;
use rppal::gpio::{Gpio, InputPin, Trigger};
use crate::CurrentValues;

use crate::ring_buffer::RingBuffer;

pub struct GpioSignals{
    gpio : Gpio,
    pin_wheel : InputPin,
    pin_roller : InputPin,
}

impl GpioSignals {
    pub fn new() -> Result<GpioSignals, Box<dyn Error>> {
        let gpio = Gpio::new()?;
        let pin_wheel = gpio.get(23)?.into_input_pulldown();
        let pin_roller = gpio.get(24)?.into_input_pulldown();

        Ok(GpioSignals {
            gpio,
            pin_wheel,
            pin_roller,
        })
    }

    pub fn start(&mut self,
                 signals_wheel  : Arc<Mutex<RingBuffer>>,
                 signals_roller : Arc<Mutex<RingBuffer>>,
                 values         : CurrentValues ) -> Result<(), Box<dyn Error>> {

        let sum_wheel = values.sum_signals_wheel.clone();
        let sum_roller = values.sum_signals_roller.clone();

        self.pin_wheel .set_async_interrupt(Trigger::FallingEdge,move |_level| {
            #[cfg(debug_assertions)]
            println!("Signal: WHEEL");
            signals_wheel.lock().unwrap().push(Instant::now());
            sum_wheel.fetch_add(1, Ordering::Relaxed);
        })?;
        self.pin_roller.set_async_interrupt(Trigger::FallingEdge,move |_level| {
            #[cfg(debug_assertions)]
            println!("Signal: ROLLER");
            signals_roller.lock().unwrap().push(Instant::now());
            sum_roller.fetch_add(1, Ordering::Relaxed);
        })?;
        Ok(())
    }
}
