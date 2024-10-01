use std::collections::HashMap;
use std::fmt::format;
use wasm_bindgen::JsValue;
use web_sys::console;
use crate::Player;
use crate::map;
use crate::Tile;

pub enum DirectionHV {
    Horizontal,
    Vertical
}
pub fn tile_collision(
    tuple: (usize, usize), 
    collision_map: &HashMap<(usize, usize), Tile>
) -> (bool,Option<&map::Tile>) {
    (collision_map.contains_key(&tuple),collision_map.get(&tuple))
}
pub fn manage_hv_collision(
    t1_t2_off: ((usize, usize), (usize, usize), f64), 
    collision_map: &HashMap<(usize, usize), Tile>,
    player: &mut Player,
    direction: DirectionHV
) {
    let mut new_position = |t:&Tile| {
        match direction {
            DirectionHV::Vertical => player.position.y = t.position.y + t1_t2_off.2,
            DirectionHV::Horizontal => player.position.x = t.position.x + t1_t2_off.2
        }
    };
    match direction {
        DirectionHV::Vertical => player.velocity.y = 0.,
        DirectionHV::Horizontal => player.velocity.x = 0.
    }
    let tile1 = tile_collision(t1_t2_off.0, collision_map).1;
    let tile2 = tile_collision(t1_t2_off.1, collision_map).1;
    if let Some(t) = tile1 {
        new_position(t);
    } else if let Some(t) = tile2 {
        new_position(t);
    }
    //console::log_1(&JsValue::from_str(&format!("player {:?}: ", player)))
}

pub fn manage_diagonal_collision(
    ntuple: ((usize, usize), (usize, usize), (usize, usize), f64, f64, f64, f64, f64, f64), 
    collision_map: &HashMap<(usize, usize), Tile>,
    player: &mut Player,
) {
    let (
        main_tile,
        second_tile,
        third_tile,
        off_tile_x,
        off_tile_y,
        off_tile_x_intersection,
        off_tile_y_intersection,
        off_player_x,
        off_player_y
    ) = ntuple;

    if tile_collision(main_tile, &collision_map).0 {
        //console::log_1(&JsValue::from_str(""));
        if tile_collision(second_tile, &collision_map).0 {
            //console::log_1(&JsValue::from_str(""));
            player.velocity.y = 0.;
            if let Some(t) = tile_collision(second_tile, &collision_map).1 {
                player.position.y = t.position.y + off_tile_y
            }
        } 
        if tile_collision(third_tile, &collision_map).0 {
            //console::log_1(&JsValue::from_str(""));
            player.velocity.x = 0.;
            if let Some(t) = tile_collision(third_tile, &collision_map).1 {
                player.position.x = t.position.x + off_tile_x
            }
        }
        if !tile_collision(second_tile, &collision_map).0 && !tile_collision(third_tile, &collision_map).0 {
            //console::log_1(&JsValue::from_str(""));
            let tile = tile_collision(main_tile, &collision_map).1;
            if let Some(t) = tile {
                // Calculate slope
                let m = player.velocity.y / player.velocity.x;
                // Calculate intersection y-coordinate
                let intersection_y = m * (
                    t.position.x + off_tile_x_intersection - (player.position.x + off_player_x )
                ) + (player.position.y + off_player_y);
                let from_below_above = intersection_y > t.position.y + off_tile_y_intersection;
                if from_below_above {
                    player.velocity.y = 0.;
                    player.position.y = t.position.y + off_tile_y;
                } else {
                    player.velocity.x = 0.;
                    player.position.x = t.position.x + off_tile_x;
                }
            }
        }
    } else if tile_collision(second_tile, &collision_map).0 || tile_collision(third_tile, &collision_map).0 {
        if let Some(t) = tile_collision(second_tile, &collision_map).1 {
            //console::log_1(&JsValue::from_str("top right ---"));
            player.velocity.y = 0.;
            player.position.y = t.position.y + off_tile_y
        }
        if let Some(t) = tile_collision(third_tile, &collision_map).1 {
            //console::log_1(&JsValue::from_str("top third_tile ---"));
            player.velocity.x = 0.;
            player.position.x = t.position.x + off_tile_x
        }
    }
}



pub fn manage_player_collision_with_tile(player: &mut Player, collision_map: &HashMap<(usize, usize), Tile>) {
    //tiles around the player
    let top_right = (
        ((player.position.x + player.velocity.x + 50.0) / 50.0).floor() as usize,
        ((player.position.y + player.velocity.y) / 50.0).floor() as usize
    );
    let bottom_right = (
        ((player.position.x + player.velocity.x + 50.0) / 50.0).floor() as usize,
        ((player.position.y + player.velocity.y + 50.0) / 50.0).floor() as usize
    );
    let top_left = (
        ((player.position.x + player.velocity.x) / 50.0).floor() as usize,
        ((player.position.y + player.velocity.y) / 50.0).floor() as usize
    );
    let bottom_left = (
        ((player.position.x + player.velocity.x) / 50.0).floor() as usize,
        ((player.position.y + player.velocity.y + 50.0) / 50.0).floor() as usize
    );

    if player.velocity.x == 0. && player.velocity.y == 0. { return }
   
    let collision_top_right = tile_collision(top_right, &collision_map).0; 
    let collision_top_left = tile_collision(top_left, &collision_map).0; 
    let collision_bottom_right = tile_collision(bottom_right, &collision_map).0; 
    let collision_bottom_left = tile_collision(bottom_left, &collision_map).0; 

    // Purely vertical or purely Horizontal movement
    let mut h_cases = HashMap::new(); 
    h_cases.insert(String::from("right"), (top_right, bottom_right, -50.1));
    h_cases.insert(String::from("left"), (top_left, bottom_left, 50.0));
    h_cases.insert(String::from("down"), (bottom_left, bottom_right, -50.1));
    h_cases.insert(String::from("up"), (top_left, top_right, 50.0));

    if player.velocity.y == 0. {
        if player.velocity.x > 0. && collision_top_right || collision_bottom_right {
            console::log_1(&JsValue::from_str(&format!("here")));
            manage_hv_collision(*h_cases.get("right").unwrap(), collision_map, player, DirectionHV::Horizontal)
        } else if collision_top_left || collision_bottom_left  {
            manage_hv_collision(*h_cases.get("left").unwrap(), collision_map, player, DirectionHV::Horizontal);
        }
    } else if player.velocity.x == 0. {
        if player.velocity.y > 0. && collision_bottom_left || collision_bottom_right {
            manage_hv_collision(*h_cases.get("down").unwrap(), collision_map, player, DirectionHV::Vertical)
        }
        if player.velocity.y < 0. && collision_top_left || collision_top_right {
            manage_hv_collision(*h_cases.get("up").unwrap(), collision_map, player, DirectionHV::Vertical)
        }

    //diagonal movement
    } else if player.velocity.y < 0. {
        let mut d_cases = HashMap::new();
        // off_tile_x, off_tile_y, off_tile_x_intersection, off_tile_y_intersection, off_player_x, off_player_y
        d_cases.insert(String::from("up-left"), (top_left, top_right, bottom_left, 50.0, 50.0, 0., 50.0, 0., 0.));
        d_cases.insert(String::from("up-right"), (top_right, top_left, bottom_right, -50.1, 50.0, 0., 50.0, 50., 0.));

        if player.velocity.x < 0. {
            manage_diagonal_collision(*d_cases.get("up-left").unwrap(), collision_map, player)
        } else {
            manage_diagonal_collision(*d_cases.get("up-right").unwrap(), collision_map, player);
        }
    } else if player.velocity.y > 0. {
        if player.velocity.x < 0. {
            if tile_collision(bottom_left, &collision_map).0 {
                console::log_1(&JsValue::from_str("bottom left hit"));
                if tile_collision(bottom_right, &collision_map).0 {
                    console::log_1(&JsValue::from_str("bottom"));
                    player.velocity.y = 0.;
                    if let Some(t) = tile_collision(bottom_right, &collision_map).1 {
                        player.position.y = t.position.y - 50.1
                    }
                } 
                if tile_collision(top_left, &collision_map).0 {
                    console::log_1(&JsValue::from_str("left"));
                    player.velocity.x = 0.;
                    if let Some(t) = tile_collision(top_left, &collision_map).1 {
                        player.position.x = t.position.x + 50.0
                    }
                }
                if !tile_collision(bottom_right, &collision_map).0 && !tile_collision(top_left, &collision_map).0 {
                    console::log_1(&JsValue::from_str("only bottom left"));
                    let tile = tile_collision(bottom_left, &collision_map).1;
                    if let Some(t) = tile {
                        // Calculate slope
                        let m = player.velocity.y / player.velocity.x;
                        // Calculate intersection y-coordinate
                        let intersection_y = m * ((t.position.x + 50.0) - player.position.x) + (player.position.y + 50.0);
                        let from_above = intersection_y < t.position.y;

                        if from_above {
                            player.velocity.y = 0.;
                            player.position.y = t.position.y - 50.1;
                        } else {
                            player.velocity.x = 0.;
                            player.position.x = t.position.x + 50.0
                        }
                    }
                }
            } else if tile_collision(bottom_right, &collision_map).0 || tile_collision(top_left, &collision_map).0 {
                if let Some(t) = tile_collision(bottom_right, &collision_map).1 {
                    player.velocity.y = 0.;
                    player.position.y = t.position.y - 50.1
                }
                if let Some(t) = tile_collision(top_left, &collision_map).1 {
                    player.velocity.x = 0.;
                    player.position.x = t.position.x + 50.0;
                }
            }
        } else {
            if tile_collision(bottom_right, &collision_map).0 {
                console::log_1(&JsValue::from_str("bottom right hit"));
                if tile_collision(bottom_left, &collision_map).0 {
                    console::log_1(&JsValue::from_str("bottom"));
                    player.velocity.y = 0.;
                    if let Some(t) = tile_collision(bottom_right, &collision_map).1 {
                        player.position.y = t.position.y - 50.1
                    }
                } 
                if tile_collision(top_right, &collision_map).0 {
                    console::log_1(&JsValue::from_str("right"));
                    player.velocity.x = 0.;
                    if let Some(t) = tile_collision(top_right, &collision_map).1 {
                        player.position.x = t.position.x - 50.1
                    }
                }
                if !tile_collision(bottom_left, &collision_map).0 && !tile_collision(top_right, &collision_map).0 {
                    console::log_1(&JsValue::from_str("only bottom right"));
                    let tile = tile_collision(bottom_right, &collision_map).1;
                    if let Some(t) = tile {
                        // Calculate slope
                        let m = player.velocity.y / player.velocity.x;
                        // Calculate intersection y-coordinate
                        let intersection_y = m * (t.position.x - (player.position.x + 50.0)) + player.position.y + 50.0;
                        let from_above = intersection_y < t.position.y;
                        if from_above {
                            console::log_1(&JsValue::from_str(&format!("onlyBotRight from above")));
                            player.velocity.y = 0.;
                            player.position.y = t.position.y - 50.1;
                        } else {
                            console::log_1(&JsValue::from_str(&format!("onlyBotRight NOT from above")));
                            player.velocity.x = 0.;
                            player.position.x = t.position.x - 50.1
                        }
                    }
                }
            } else if tile_collision(bottom_left, &collision_map).0 || tile_collision(top_right, &collision_map).0 {
                if let Some(t) = tile_collision(bottom_left, &collision_map).1 {
                    player.position.y = t.position.y - 50.1;
                    player.velocity.y = 0.;
                }
                if let Some(t) = tile_collision(top_right, &collision_map).1 {
                    player.velocity.x = 0.;
                    player.position.x = t.position.x - 50.1;
                }
            }
        }
    }
}






















//        if player.velocity.x < 0. {
//            if tile_collision(top_left, &collision_map).0 {
//                console::log_1(&JsValue::from_str("top left hit"));
//                if tile_collision(top_right, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("top"));
//                    player.velocity.y = 0.;
//                    if let Some(t) = tile_collision(top_right, &collision_map).1 {
//                        player.position.y = t.position.y + 50.0
//                    }
//                } 
//                if tile_collision(bottom_left, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("left"));
//                    player.velocity.x = 0.;
//                    if let Some(t) = tile_collision(bottom_left, &collision_map).1 {
//                        player.position.x = t.position.x + 50.0
//                    }
//                }
//                if !tile_collision(top_right, &collision_map).0 && !tile_collision(bottom_left, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("only top left"));
//                    let tile = tile_collision(top_left, &collision_map).1;
//                    if let Some(t) = tile {
//                        // Calculate slope
//                        let m = player.velocity.y / player.velocity.x;
//                        // Calculate intersection y-coordinate
//                        let intersection_y = m * (t.position.x + 50.0 - player.position.x) + player.position.y;
//                        let from_below = intersection_y > t.position.y + 50.0;
//                        if from_below {
//                            player.velocity.y = 0.;
//                            player.position.y = t.position.y + 50.0;
//                        } else {
//                            player.velocity.x = 0.;
//                            player.position.x = t.position.x + 50.0
//                        }
//                    }
//                }
//            } else if tile_collision(top_right, &collision_map).0 || tile_collision(bottom_left, &collision_map).0 {
//                if let Some(t) = tile_collision(top_right, &collision_map).1 {
//                    console::log_1(&JsValue::from_str("top right ---"));
//                    player.velocity.y = 0.;
//                    player.position.y = t.position.y + 50.0
//                }
//                if let Some(t) = tile_collision(bottom_left, &collision_map).1 {
//                    console::log_1(&JsValue::from_str("top bottom_left ---"));
//                    player.velocity.x = 0.;
//                    player.position.x = t.position.x + 50.0
//                }
//            }
//        } else {
//            if tile_collision(top_right, &collision_map).0 {
//                console::log_1(&JsValue::from_str("top right hit"));
//                if tile_collision(top_left, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("top"));
//                    player.velocity.y = 0.;
//                    if let Some(t) = tile_collision(top_right, &collision_map).1 {
//                        player.position.y = t.position.y + 50.0
//                    }
//                } 
//                if tile_collision(bottom_right, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("right"));
//                    player.velocity.x = 0.;
//                    if let Some(t) = tile_collision(bottom_right, &collision_map).1 {
//                        player.position.x = t.position.x - 50.1
//                    }
//                }
//                if !tile_collision(top_left, &collision_map).0 && !tile_collision(bottom_right, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("only top right"));
//                    let tile = tile_collision(top_right, &collision_map).1;
//                    if let Some(t) = tile {
//                        // Calculate slope
//                        let m = player.velocity.y / player.velocity.x;
//                        // Calculate intersection y-coordinate
//                        let intersection_y = m * (t.position.x - (player.position.x + 50.0)) + player.position.y ;
//                        let from_below = intersection_y > t.position.y + 50.0;
//                        if from_below {
//                            player.velocity.y = 0.;
//                            player.position.y = t.position.y + 50.0;
//                        } else {
//                            player.velocity.x = 0.;
//                            player.position.x = t.position.x - 50.1
//                        }
//                    }
//                }
//            } else if tile_collision(top_left, &collision_map).0 || tile_collision(bottom_right, &collision_map).0 {
//                if let Some(t) = tile_collision(top_left, &collision_map).1 {
//                    console::log_1(&JsValue::from_str("top left ---"));
//                    player.velocity.y = 0.;
//                    player.position.y = t.position.y + 50.0
//                }
//                if let Some(t) = tile_collision(bottom_right, &collision_map).1 {
//                    console::log_1(&JsValue::from_str("bottom right ---"));
//                    player.velocity.x = 0.;
//                    player.position.x = t.position.x - 50.01
//                }
//            }
//        }
//    } else if player.velocity.y > 0. {
//        if player.velocity.x < 0. {
//            if tile_collision(bottom_left, &collision_map).0 {
//                console::log_1(&JsValue::from_str("bottom left hit"));
//                if tile_collision(bottom_right, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("bottom"));
//                    player.velocity.y = 0.;
//                    if let Some(t) = tile_collision(bottom_right, &collision_map).1 {
//                        player.position.y = t.position.y - 50.1
//                    }
//                } 
//                if tile_collision(top_left, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("left"));
//                    player.velocity.x = 0.;
//                    if let Some(t) = tile_collision(top_left, &collision_map).1 {
//                        player.position.x = t.position.x + 50.0
//                    }
//                }
//                if !tile_collision(bottom_right, &collision_map).0 && !tile_collision(top_left, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("only bottom left"));
//                    let tile = tile_collision(bottom_left, &collision_map).1;
//                    if let Some(t) = tile {
//                        // Calculate slope
//                        let m = player.velocity.y / player.velocity.x;
//                        // Calculate intersection y-coordinate
//                        let intersection_y = m * ((t.position.x + 50.0) - player.position.x) + (player.position.y + 50.0);
//                        let from_above = intersection_y < t.position.y;
//
//                        if from_above {
//                            player.velocity.y = 0.;
//                            player.position.y = t.position.y - 50.1;
//                        } else {
//                            player.velocity.x = 0.;
//                            player.position.x = t.position.x + 50.0
//                        }
//                    }
//                }
//            } else if tile_collision(bottom_right, &collision_map).0 || tile_collision(top_left, &collision_map).0 {
//                if let Some(t) = tile_collision(bottom_right, &collision_map).1 {
//                    player.velocity.y = 0.;
//                    player.position.y = t.position.y - 50.1
//                }
//                if let Some(t) = tile_collision(top_left, &collision_map).1 {
//                    player.velocity.x = 0.;
//                    player.position.x = t.position.x + 50.0;
//                }
//            }
//        } else {
//            if tile_collision(bottom_right, &collision_map).0 {
//                console::log_1(&JsValue::from_str("bottom right hit"));
//                if tile_collision(bottom_left, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("bottom"));
//                    player.velocity.y = 0.;
//                    if let Some(t) = tile_collision(bottom_right, &collision_map).1 {
//                        player.position.y = t.position.y - 50.1
//                    }
//                } 
//                if tile_collision(top_right, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("right"));
//                    player.velocity.x = 0.;
//                    if let Some(t) = tile_collision(top_right, &collision_map).1 {
//                        player.position.x = t.position.x - 50.1
//                    }
//                }
//                if !tile_collision(bottom_left, &collision_map).0 && !tile_collision(top_right, &collision_map).0 {
//                    console::log_1(&JsValue::from_str("only bottom right"));
//                    let tile = tile_collision(bottom_right, &collision_map).1;
//                    if let Some(t) = tile {
//                        // Calculate slope
//                        let m = player.velocity.y / player.velocity.x;
//                        // Calculate intersection y-coordinate
//                        let intersection_y = m * (t.position.x - (player.position.x + 50.0)) + player.position.y + 50.0;
//                        let from_above = intersection_y < t.position.y;
//                        if from_above {
//                            console::log_1(&JsValue::from_str(&format!("onlyBotRight from above")));
//                            player.velocity.y = 0.;
//                            player.position.y = t.position.y - 50.1;
//                        } else {
//                            console::log_1(&JsValue::from_str(&format!("onlyBotRight NOT from above")));
//                            player.velocity.x = 0.;
//                            player.position.x = t.position.x - 50.1
//                        }
//                    }
//                }
//            } else if tile_collision(bottom_left, &collision_map).0 || tile_collision(top_right, &collision_map).0 {
//                if let Some(t) = tile_collision(bottom_left, &collision_map).1 {
//                    player.position.y = t.position.y - 50.1;
//                    player.velocity.y = 0.;
//                }
//                if let Some(t) = tile_collision(top_right, &collision_map).1 {
//                    player.velocity.x = 0.;
//                    player.position.x = t.position.x - 50.1;
//                }
//            }
//        }
//    }
//}
//
