use crate::{Player, ENEMIES};

pub fn enemies_check_collisions(player: &mut Player) {
    let mut enemies = ENEMIES.lock().unwrap();

    for enemy in enemies.iter_mut() {
        enemy.check_collision(player);
    }
}
