// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    const RESULT: i32 = 40;

    // return expected minutes in the oven
    return RESULT
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let expected = expected_minutes_in_oven();

    // calculate remaining minutes in oven given actual minutes in oven:
    let result = &expected - &actual_minutes_in_oven;

    return result
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    const TIME_PER_LAYER: i32 = 2;

    // calculate preparation time in minutes for number of layers:
    let result = &number_of_layers * &TIME_PER_LAYER;

    return result
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    // unimplemented!(
    //     "",
    //     number_of_layers,
    //     actual_minutes_in_oven
    // )
    let prep_time = preparation_time_in_minutes(number_of_layers);

    // calculate elapsed time in minutes for number of layers and actual minutes in oven
    let result = &prep_time + actual_minutes_in_oven;

    return result
}
