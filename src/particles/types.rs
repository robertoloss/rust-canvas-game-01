use crate::{log_out_f, Vec2};

#[derive(Debug)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub velocity_change: Vec2,
    pub color: String,
    pub active: bool,
    pub counter: u64,
    pub limit: u64
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            position: Vec2 { x: 0.0, y: 0.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            velocity_change: Vec2 { x: 0.0, y: 0.0 },
            color: "white".to_string(),
            active: true,  
            counter: 0,    
            limit: 50,
        }
    }
}



impl Particle {
    pub fn moves(&mut self) {
        if self.counter >= self.limit {
            self.active = false;
            return
        }
        self.velocity.x += self.velocity_change.x;
        self.velocity.y += self.velocity_change.y;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        self.color = darken_color(&self.color);

        self.counter += 1;
    }
}

fn darken_color(hex: &str) -> String {
    let part = &hex[1..3];
    let value = u32::from_str_radix(part, 16).unwrap(); 
    let darker = if value > 20 { value - 3 } else { value }; 
    format!("#{:02X}{:02X}{:02X}", darker, darker, darker)
}
