use crate::{get_random, get_random_int, Player, Vec2};
use super::{types::Particle, utils::random_gray};


pub fn hit_ground_particles(player: &Player, particles: &mut Vec<Particle>) {

    let num_part = get_random_int(10, 12);

    for _ in 0..num_part {
        let off_x = get_random(-1., 2.);
        let off_limit = get_random_int(1, 4);
        let abs_vel_x = get_random(0.7, 1.6);
        let vel_x = if off_x < 1. { -abs_vel_x } else { abs_vel_x };
        let vel_y = get_random(-0.2, -0.7);  

        let p = Particle {
            position: Vec2 { 
                x: player.position.x + 18. + off_x, 
                y: player.position.y + 42. //24.
            },
            velocity: Vec2 { 
                x: vel_x,  
                y: vel_y  
            },
            color: random_gray(190,220),
            limit: 20 + off_limit as u64,
            velocity_change: Vec2 {
                x: 0.,
                y: (vel_y * -1.) / 11. 
            },
            ..Default::default()
        };
        particles.push(p);
    }
}

