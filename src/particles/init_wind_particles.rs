use crate::{utils::{extern_c::{get_random, get_random_int}}, Vec2};
use super::{types::Particle, utils::{random_gray}};

pub fn init_wind_particles(
    particles: &mut Vec<Particle>
) {
    let num_part = get_random_int(20, 40);
    for _ in 0..num_part {
        let pos_x = get_random(48., 800.);
        let pos_y = get_random(48., 800.);
        let color = random_gray(10, 60);
        let vel_x = get_random(1., 3.);
        let vel_y = get_random(-3., 3.);
        let change_vel_y = -(vel_y /500.);
        let p = Particle {
            in_front: false,
            position: Vec2 { 
                x: pos_x, 
                y: pos_y
            },
            velocity: Vec2 { 
                x: vel_x,   
                y: 0.  
            },
            velocity_change: Vec2 {
                x: 0.,
                y: change_vel_y
            },
            color: color.clone(),
            color_change: Some(color.to_string()),
            limit: 300,
            darken: false,
            ..Default::default()
        };
        particles.push(p);
    }
}
