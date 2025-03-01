use crate::{get_random, get_random_int, Player, Vec2, PARTICLES};
use super::types::Particle;

fn random_gray() -> String {
    let gray_value = get_random_int(100, 200); 
    format!("#{:02X}{:02X}{:02X}", gray_value, gray_value, gray_value) 
}

pub fn generate_jump_particles(player: &Player) {
    let mut particles = PARTICLES.lock().unwrap();
    let pos_x;
    let mut vel_x = 1.;
    if player.facing_left {
        pos_x = 30.
    } else if player.facing_right {
        pos_x = 12.;
        vel_x = -vel_x
    } else {
        pos_x = 0.
    }

    let num_part = get_random_int(10, 20);
    for _ in 0..num_part {
        let off_x = get_random(-2., 2.);
        let off_y = get_random(-0.2, 1.8);
        let off_limit = get_random_int(1, 20);
        let p = Particle {
            position: Vec2 { 
                x: player.position.x + pos_x, 
                y: player.position.y + 12. //24.
            },
            velocity: Vec2 { 
                x: vel_x + off_x,  
                y: 0.5 + off_y  
            },
            color: random_gray(),
            limit: 15 + off_limit as u64,
            ..Default::default()
        };
        particles.push(p);
    }
}

