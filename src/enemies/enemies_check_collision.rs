use crate::Player;
use super::types::EnemyTrait;

pub fn enemies_check_collisions(player: &mut Player, enemies: &mut Vec<Box<dyn EnemyTrait>>) {
    for enemy in enemies.iter_mut() {
        enemy.check_collision(player);
    }
}
