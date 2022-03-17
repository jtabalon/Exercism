// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const RATE_ONE: f64 = 1.0;
const RATE_TWO: f64 = 0.9;
const RATE_THREE: f64 = 0.77;


pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)

    // TODO: troubleshoot types + functions

    let prod_rate: f64 = 
        if speed <= 5 {
            4.0 * speed
        }
        else if speed >= 5 && speed <= 8 {
            4.0 * speed * 0.90
        }
        else if speed >= 9 && speed <= 10 {
            4.0 * speed * 0.77
        };
    
    return prod_rate;

}

pub fn working_items_per_minute(speed: u8) -> u32 {

    let items_per_minute: u32 = production_rate_per_hour(speed) as u32 / 60;

    return items_per_minute;
    
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
}
