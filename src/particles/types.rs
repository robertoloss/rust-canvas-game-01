use crate::{log_out_f, Vec2};

#[derive(Debug)]
enum ColorChange {
    Darken,
    Shift(String)
}

#[derive(Debug)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub velocity_change: Vec2,
    pub color: String,
    pub active: bool,
    pub counter: u64,
    pub limit: u64,
    pub behavior: ColorChange
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            position: Vec2 { x: 0.0, y: 0.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            velocity_change: Vec2 { x: 0.0, y: 0.0 },
            color: "#ffffff".to_string(),
            active: true,  
            counter: 0,    
            limit: 50,
            behavior: ColorChange::Darken
        }
    }
}

impl Particle {
    pub fn moves(&mut self, delta: f64) {
        if self.counter >= self.limit {
            self.active = false;
            return
        }
        self.velocity.x += self.velocity_change.x;
        self.velocity.y += self.velocity_change.y;

        self.position.x += self.velocity.x / delta;
        self.position.y += self.velocity.y / delta;

        match &self.behavior {
            ColorChange::Darken => { 
                self.color = darken_color(&self.color);
            }
            ColorChange::Shift(target_color) => { 
                self.color = shift_towards_color(&self.color, &target_color);
            }
        }

        self.counter += 1;
    }
}

fn darken_color(hex: &str) -> String {
    if !hex.starts_with('#') || hex.len() != 7 {
        return "#FFFFFF".to_string(); // Fallback if input is invalid
    }
    let r = u32::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u32::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u32::from_str_radix(&hex[5..7], 16).unwrap();

    let darken = |v: u32| if v > 20 { v - 3 } else { v };

    let new_r = darken(r);
    let new_g = darken(g);
    let new_b = darken(b);

    format!("#{:02X}{:02X}{:02X}", new_r, new_g, new_b)
}

fn shift_towards_color(current_hex: &str, target_hex: &str) -> String {
    if !current_hex.starts_with('#') || 
        current_hex.len() != 7 || 
        !target_hex.starts_with('#') || 
        target_hex.len() != 7 
    {
        return "#FFFFFF".to_string(); // Fallback if input is invalid
    }
    let current_r = u32::from_str_radix(&current_hex[1..3], 16).unwrap();
    let current_g = u32::from_str_radix(&current_hex[3..5], 16).unwrap();
    let current_b = u32::from_str_radix(&current_hex[5..7], 16).unwrap();

    let target_r = u32::from_str_radix(&target_hex[1..3], 16).unwrap();
    let target_g = u32::from_str_radix(&target_hex[3..5], 16).unwrap();
    let target_b = u32::from_str_radix(&target_hex[5..7], 16).unwrap();

    let shift = |current: u32, target: u32| {
        if current < target {
            current + 1
        } else if current > target {
            current - 1
        } else {
            current
        }
    };

    let new_r = shift(current_r, target_r);
    let new_g = shift(current_g, target_g);
    let new_b = shift(current_b, target_b);

    format!("#{:02X}{:02X}{:02X}", new_r, new_g, new_b)
}


