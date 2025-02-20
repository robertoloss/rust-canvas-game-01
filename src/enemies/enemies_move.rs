use crate::ENEMIES;


pub fn enemies_move() {
    let mut enemies = ENEMIES.lock().unwrap();

    for enemy in enemies.iter_mut() {
        enemy.moves();
    }
}
