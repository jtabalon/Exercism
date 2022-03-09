// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    // return expected minutes in the oven
   40 
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    // "calculate remaining minutes in oven given actual minutes in oven: {}", actual_minutes_in_oven
    expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    // each layer takes 2 minutes to prepare
    number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    // return sum of prep time and time lasagna has spent in the oven
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
