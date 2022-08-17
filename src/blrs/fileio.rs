/**
 * fileio.rs -- Handles reading and writing to the sysfs backlight
 */

use std::{fs, io::Write};
use super::Err;

#[cfg(feature = "intel")]
const BL_DIR: &str = "/sys/class/backlight/intel_backlight/";
#[cfg(not(feature = "intel"))]
const BL_DIR: &str = "/sys/class/backlight/acpi_video0/";

/* 
 * Reads the the max backlight value from the OS
 */
pub fn read_max_brightness() -> Result<usize, Err> {
    fs::read_to_string(BL_DIR.to_owned() + "max_brightness")
        .map_err(|e| Err::File(e))
        .and_then(|s| s.trim().parse().map_err(|_| Err::ValueParse))
}

/* 
 * Reads the the current backlight value from the OS
 */
pub fn read_cur_brightness() -> Result<usize, Err> {
    fs::read_to_string(BL_DIR.to_owned() + "brightness")
        .map_err(|e| Err::File(e))
        .and_then(|s| s.trim().parse().map_err(|_| Err::ValueParse))
}

/*
 * Writes specified backlight percentage to the OS
 * ASSUMES VALUE IS VALID FOR READ ECONOMY
 */
pub fn write_cur_brightness(value: usize) -> Result<(), Err> {

    // Opening File
    let mut file = fs::File::create(BL_DIR.to_owned() + "brightness")
        .map_err(|e| Err::File(e))?;

    // Writing
    file.write(&value.to_string().as_bytes())
        .map_err(|e| Err::File(e))?;

    Ok(())
}