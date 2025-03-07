use crate::Player;
use super::types::Coin;

pub fn manage_coins(player: &mut Player, coins: &mut Vec<Coin>) {
    coins
        .iter_mut()
        .for_each(|coin| {
            if coin.player_collision(player) {
                coin.deactivate()
            }
        });
    coins
        .retain(|coin| coin.active);
}
