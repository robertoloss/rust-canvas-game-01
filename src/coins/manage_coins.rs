use crate::Player;
use super::types::Coin;

pub fn manage_coins(player: &mut Player, coins: &mut Vec<Coin>) {
    coins
        .iter_mut()
        .for_each(|coin| {
            if coin.player_collision(player) {
                coin.show_plus_one()
            }
            if coin.show_plus_one {
                if coin.counter < 50 {
                    coin.tile.position.y -= 1.;
                    coin.counter += 1;
                } else {
                    coin.deactivate();
                }
            }
        });
    coins
        .retain(|coin| coin.active);
}
