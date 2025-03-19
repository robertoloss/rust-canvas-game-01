use crate::{utils::extern_c::{get_random, get_random_int}, Player, Vec2};
use super::{types::Particle, utils::random_yellow};


pub fn lava_particles(
    particles: &mut Vec<Particle>,
    tile_position: Vec2
) {
    let num_part = get_random_int(4, 8);
    for _ in 0..num_part {
        let off_limit = get_random_int(0, 20);
        let pos_x = 24. + tile_position.x + get_random(-24., 20.);
        let pos_y = tile_position.y + 36.;
        let vel_y = get_random(-0.2, -0.6);
        let p = Particle {
            should_die: false,
            position: Vec2 { 
                x: pos_x, 
                y: pos_y
            },
            init_position: Vec2 { 
                x: pos_x, 
                y: pos_y
            },
            velocity: Vec2 { 
                x: 0.,   
                y: vel_y  
            },
            init_velocity: Vec2 { 
                x: 0.,   
                y: vel_y  
            },
            color: random_yellow(140, 180),
            init_color: random_yellow(180, 220),
            color_change: Some("#ff441c".to_string()),
            limit: 80 + off_limit as u64,
            darken: true,
            ..Default::default()
        };
        particles.push(p);
    }
}
