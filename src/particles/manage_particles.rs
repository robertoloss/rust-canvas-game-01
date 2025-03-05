use super::types::Particle;


pub fn manage_particles(particles: &mut Vec<Particle>) {
    particles
        .iter_mut()
        .for_each(|particle| {
            particle.moves();
        });
    particles
        .retain(|p| p.active);
}
