/**
 * blrs.rs -- Parent module that handles subcommand execution
 */
mod fileio;
mod err;
mod helpers;
mod cli;

use fileio::*;
use helpers::*;
use err::Err;
pub use cli::parse_args;

use std::time;

struct BLRS {
    max: usize
}

impl BLRS {
    fn new() -> Result<Self, Err> {
        Ok(Self { max: read_max_brightness()? })
    }

    /**
     * Get Subcommand
     */
    fn sc_get(&self) -> Result<(), Err> {
        println!("{}", inv_scale_percent(read_cur_brightness()?, self.max)?);
        Ok(())
    }

    /**
     * Set Subcommand
     */
    fn sc_set(&self, perc: usize) -> Result<(), Err> {
        let new_value = scale_percent(perc, self.max)?;
        if new_value > self.max {
            Err(Err::OutOfBounds)
        } else {
            write_cur_brightness(new_value)
        }
    }

    /**
     * Inc Subcommand
     */
    fn sc_inc(&self, perc: usize) -> Result<(), Err> {
        let new_value = read_cur_brightness()? + scale_percent(perc, self.max)?;
        if new_value > self.max {
            Err(Err::OutOfBounds)
        } else {
            write_cur_brightness(new_value)
        }
    }

    /**
     * Dec Subcommand
     */
    fn sc_dec(&self, perc: usize) -> Result<(), Err> {
        let new_value = safe_sub(read_cur_brightness()?, scale_percent(perc, self.max)?)?;
        write_cur_brightness(new_value)
    }

    /**
     * Smooth Subcommand
     */
    fn sc_smooth(&self, perc: usize, t: usize) -> Result<(), Err> {
        if t == 0 {
            return Err(Err::OutOfBounds);
        }

        // Decoding current and stop values
        let start = read_cur_brightness()? as f32;
        let stop = scale_percent(perc, self.max)? as f32;

        // Finding change in backlight per step
        let diff = if start == stop {
            // Nothing to do
            return Ok(())
        } else {
            // Positive if increasing, negative else
            (stop - start) as f32
        };

        // Starting timer
        let max_dur = t as f32;
        let mut time;
        let now = time::Instant::now();

        loop {
            time = now.elapsed();
            let new_val = (start + time.as_millis() as f32 / max_dur * diff) as usize;

            if new_val > self.max {
                break;
            }
            write_cur_brightness(new_val)?;
        }

        // Final set to ensure no rounding error
        self.sc_set(perc)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smooth_timing() {
        let blrs = BLRS::new().expect("test_smooth_timing: failed to init blrs");

        let ms = 2000;

        // Initialize to 0%
        blrs.sc_set(0).expect("test_smooth_timing: failed to set");

        // Start timer
        let now = time::Instant::now();

        // Smooth to 100%
        blrs.sc_smooth(100, ms).expect("test_smooth_timing: failed to smooth");

        // End timer
        let elapsed = now.elapsed();

        assert_eq!(elapsed.as_millis(), ms as u128);
    }
}