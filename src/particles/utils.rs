use crate::{log_out_f, utils::extern_c::get_random_int};


pub fn random_gray(min: u32, max: u32) -> String {
    let gray_value = get_random_int(min, max); 
    format!("#{:02X}{:02X}{:02X}", gray_value, gray_value, gray_value) 
}

pub fn random_yellow(min: u32, max: u32) -> String {
    let red = get_random_int(min, max);   // High Red component
    let green = get_random_int(min, max); // High Green component
    let blue = get_random_int(0, min / 2); // Keep Blue low to maintain yellow tone

    let color = format!("#{:02X}{:02X}{:02X}", red, green, blue);
    //log_out_f(&color);
    color
}

