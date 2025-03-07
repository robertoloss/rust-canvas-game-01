use super::types::Particle;


pub fn manage_particles(
    particles: &mut Vec<Particle>,
    delta: f64
) {
    particles
        .iter_mut()
        .for_each(|particle| {
            particle.moves(delta);
        });
    particles
        .retain(|p| p.active);
}
