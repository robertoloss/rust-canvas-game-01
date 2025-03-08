use crate::{play, Player};
use super::types::Coin;

pub fn manage_coins(player: &mut Player, coins: &mut Vec<Coin>) {
    coins
        .iter_mut()
        .for_each(|coin| {
            let coin_in_screen = coin.map_origin == player.map_origin;
            if coin_in_screen {
                if coin.player_collision(player) {
                    if !coin.show_plus_one {
                        play("coin");
                    }
                    coin.show_plus_one()
                }
                if coin.show_plus_one {
                    if coin.counter < 50 {
                        coin.position.y -= 1.;
                        coin.counter += 1;
                    } else {
                        coin.deactivate();
                    }
                }
            }
        });
    coins
        .retain(|coin| coin.active);
}
