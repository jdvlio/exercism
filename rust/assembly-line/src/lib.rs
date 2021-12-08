// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // determine probability of success
    let p = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };

    // speed in units per hour
    let speed = 221.0 * f64::from(speed);

    // calculate hourly production rate at speed
    let result = &p * &speed;

    return result
}

pub fn working_items_per_minute(speed: u8) -> u32 {

    //calculate the amount of working items at speed
    let result = (production_rate_per_hour(speed) / 60.0) as u32;

    return result
}
