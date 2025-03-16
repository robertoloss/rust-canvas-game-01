use crate::{log_out_f, utils::extern_c::{get_random, get_random_int}, Player, Vec2, PARTICLES};
use super::{types::Particle, utils::random_yellow};


pub fn spawn_particles(
    player: &Player,
    particles: &mut Vec<Particle>
) {

    let should_p_be_generated = get_random_int(0, 10);
    if should_p_be_generated < 5 {
        return
    }
    let num_part = get_random_int(0, 1);
    for _ in 0..num_part {
        let off_limit = get_random_int(0, 20);
        let p = Particle {
            position: Vec2 { 
               x: 24. + player.position_spawn.x + get_random(-20., 17.), 
                y: player.position_spawn.y + 48. //24.
            },
            velocity: Vec2 { 
                x: 0., // get_random(-0.1 , 0.2),  
                y: get_random(-0.4, -0.9)  
            },
            color: random_yellow(140, 180),//"#ffffff".to_string(), //random_yellow(50,200),
            limit: 60 + off_limit as u64,
            ..Default::default()
        };
        particles.push(p);
    }
}

