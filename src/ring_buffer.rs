use std::ops::Sub;
use std::time::{Duration, Instant};

pub struct RingBuffer {
    buf : Vec<Instant>,
    write_idx: usize,
    values_inserted : usize,
}

impl RingBuffer {

    pub fn signals_within_duration(&self, now : &Instant, duration : &Duration) -> usize {

        if self.values_inserted == 0 {
            return 0
        }

        let max_number_items_to_traverse =
            if self.values_inserted < self.buf.len() {
                self.values_inserted
            }
            else {
                self.buf.len()
            };

        let mut i  = self.write_idx - 1;
        let mut number_signals : usize = 0;

        for _ in 0..max_number_items_to_traverse {
            let diff = now.sub(self.buf[i]);
            if diff > *duration {
                break;
            }
            number_signals += 1;
            if i == 0 {
                i = self.buf.len() - 1;
            }
            else {
                i -= 1;
            }
        }
        number_signals
    }

    pub fn push(&mut self, val : Instant) {
        if self.write_idx == self.buf.len()
        {
            self.write_idx = 0;
        }

        self.buf[self.write_idx] = val;
        self.write_idx += 1;

        self.values_inserted += 1;
    }

    pub fn new(buf_size : usize) -> RingBuffer {
        let mut _buf :Vec<Instant>= Vec::with_capacity(buf_size);
        let init = Instant::now();
        _buf.resize(buf_size, init);
        RingBuffer {
            buf : _buf,
            write_idx: 0,
            values_inserted : 0
        }
    }
}
