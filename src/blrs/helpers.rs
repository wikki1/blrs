/**
 * helpers.rs -- Bits n' bobs n' Bobs n' bits
 */
use super::Err;

/**
 * Performs subtraction with overflow detection
 */
pub fn safe_sub(x: usize, y: usize) -> Result<usize, Err> {
    if y <= x {
        Ok(x - y)
    } else {
        Err(Err::OutOfBounds)
    }
}

/**
 * Scales a number to specified percentage
 */
pub fn scale_percent(perc: usize, max: usize) -> Result<usize, Err> {
    // Out of Bounds Check
    if perc > 100 {
        Err(Err::OutOfBounds)
    } else {
        Ok(perc * max / 100)
    }
}

/**
 * Performs the inverse of scale_percent
 * Returns the percentage associated with a specific value and max
 */
pub fn inv_scale_percent(value: usize, max: usize) -> Result<usize, Err> {
    // Illegal Use, value should never exceed max
    if value > max {
        Err(Err::IllegalUse(String::from("Brightness value can never exceed maximum")))
    } else {
        Ok(value * 100 / max)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test() {
        println!("{}", inv_scale_percent(0, 2000).expect("Ahhh"));
    }
}