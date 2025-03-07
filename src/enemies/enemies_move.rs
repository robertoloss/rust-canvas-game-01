use super::types::EnemyTrait;

pub fn enemies_move(
    enemies: &mut Vec<Box<dyn EnemyTrait>>,
    delta: f64
) {
    for enemy in enemies.iter_mut() {
        enemy.moves(delta);
    }
}
