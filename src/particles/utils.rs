use crate::utils::extern_c::get_random_int;


pub fn random_gray(min: u32, max: u32) -> String {
    let gray_value = get_random_int(min, max); 
    format!("#{:02X}{:02X}{:02X}", gray_value, gray_value, gray_value) 
}
