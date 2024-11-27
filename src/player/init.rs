use std::collections::HashMap;
use crate::Vec2usize;
use crate::collisions::types::LeftRight;
use crate::player::types::*;

impl Default for Player {
    fn default() -> Self {
        let tile_size = 48.0;
        let initial_spawn = Vec2 {
            x: tile_size * 8.,
            y: tile_size * 14.,
        };
        Player {
            collision_map: None,
            position: initial_spawn.clone(),
            is_dead: false,
            position_spawn: initial_spawn,
            hitbox: Direction {
                left: 6. * 2.0, // pixel * sprite-pixel
                right: 6. * 2.0,
            },
            velocity: Vec2 {
                x: 0.0,
                y: 0.0
            },
            gravity: 0.2, //0.5,
            sprite_counter: 0,
            jump_velocity: -5.7, //-6.4, //-10.1,
            horizontal_velocity: 2.,
            max_fall_velocity: 35.,
            //is_on_the_ground: true,
            moves: Moves {
                left: false,
                right: false,
                jump: false,
                airborne: false,
                stop_jump: false,
            },
            facing_right: true,
            map_origin: Vec2usize {
                x: 0,
                y: 0
            },
            tile_size,
            screen_tiles: 16,
            can_cling: LeftRight::None,
            wants_to_cling: false,
            delta: 0.0,
            is_clinging: false,
            images: HashMap::from([
                (String::from("tile"), ThreadSafeImage(None)),
                (String::from("player"), ThreadSafeImage(None)),
                (String::from("player_left"), ThreadSafeImage(None)),
                (String::from("player_cling"), ThreadSafeImage(None)),
                (String::from("player_cling_left"), ThreadSafeImage(None)),
            ]),
            sprite_sheets: HashMap::from([
                (String::from("lava"), 
                 SpriteSheet {
                    sheet: ThreadSafeImage(None),
                    pointer_y: 0.,
                    tile_position_pointer_y: 0.,
                    counter: 0,
                    counter_limit: 8,
                    pointer_y_limit: 8. * tile_size,
                }),
                (String::from("sand"), 
                 SpriteSheet {
                    sheet: ThreadSafeImage(None),
                    pointer_y: 0.,
                    tile_position_pointer_y: 0.,
                    counter: 0,
                    counter_limit: 6,
                    pointer_y_limit: 6. * tile_size,
                }),
                (String::from("death"), 
                 SpriteSheet {
                    sheet: ThreadSafeImage(None),
                    pointer_y: 0.,
                    tile_position_pointer_y: 0.,
                    counter: 0,
                    counter_limit: 1,
                    pointer_y_limit: 20. * tile_size,
                }),
                (String::from("player_run_right"), 
                 SpriteSheet {
                    sheet: ThreadSafeImage(None),
                    pointer_y: 0.,
                    tile_position_pointer_y: 0.,
                    counter: 0,
                    counter_limit: 4,
                    pointer_y_limit: 8. * tile_size,
                }),
                (String::from("player_run_left"), 
                 SpriteSheet {
                    sheet: ThreadSafeImage(None),
                    pointer_y: 0.,
                    tile_position_pointer_y: 0.,
                    counter: 0,
                    counter_limit: 4,
                    pointer_y_limit: 8. * tile_size,
                }),
            ]),
        }
    }
}
