use bevy::{math::Vec3, prelude::{Res, Transform}};

use crate::space_core::{components::pawn::FacingDirection, resources::gridmap_main::GridmapMain};

use super::gridmap_functions::world_to_cell_id;

pub fn entity_spawn_position_for_player(
    player_transform : Transform,
    player_facing_direction : &FacingDirection,
    gridmap_main : &Res<GridmapMain>,
) -> Transform {

    let mut original_transform = player_transform.clone();

    if original_transform.translation.y < 0.1 {
        original_transform.translation.y = 0.1;
    }

    let mut new_transform = original_transform.clone();

    

    new_transform.translation += get_offset(player_facing_direction, OFFSET_CHECK);

    let cell_id = world_to_cell_id(new_transform.translation);

    match gridmap_main.data.get(&cell_id) {
        Some(cell_data) => {
            if cell_data.item != -1 {

                let mut found_correct_spawn = false;

                for i in 0..8 {

                    let this_direction;

                    new_transform = original_transform.clone();
            
                    if i == 0 {
                        this_direction = FacingDirection::UpLeft;
                    } else if i == 1 {
                        this_direction = FacingDirection::Up;
                    } else if i == 2 {
                        this_direction = FacingDirection::UpRight;
                    } else if i == 3 {
                        this_direction = FacingDirection::Right;
                    } else if i == 4 {
                        this_direction = FacingDirection::DownRight;
                    } else if i == 5 {
                        this_direction = FacingDirection::Down;
                    } else if i == 6 {
                        this_direction = FacingDirection::DownLeft;
                    } else {
                        this_direction = FacingDirection::Left;
                    }

                    new_transform.translation += get_offset(&this_direction, OFFSET_CHECK);
                
                    let cell_id = world_to_cell_id(new_transform.translation);
            
                    match gridmap_main.data.get(&cell_id) {
                        Some(cell_data) => {
                            if cell_data.item == -1 {
                                new_transform = original_transform.clone();
                                new_transform.translation += get_offset(&this_direction, OFFSET_FROM_PLAYER);
                                found_correct_spawn = true;
                                break;
                            }
                        },
                        None => {
                            new_transform = original_transform.clone();
                            new_transform.translation += get_offset(&this_direction, OFFSET_FROM_PLAYER);
                            found_correct_spawn = true;
                            break;
                        },
                    }
            
                }



                if found_correct_spawn == false {

                    new_transform = original_transform.clone();
                    new_transform.translation += 0.1 * get_offset(player_facing_direction, OFFSET_FROM_PLAYER);

                }




            }
        },
        None => {
            new_transform = original_transform.clone();
            new_transform.translation += get_offset(player_facing_direction, OFFSET_FROM_PLAYER);
        },
    }
    
    new_transform
}


const OFFSET_FROM_PLAYER : f32 = 1.;
const OFFSET_CHECK : f32 = 1.80;

fn get_offset(
    player_facing_direction : &FacingDirection,
    offset : f32,
) -> Vec3 {

    match player_facing_direction {
        FacingDirection::UpLeft => {
            Vec3::new(offset,0.,offset)
        },
        FacingDirection::Up => {
            Vec3::new(0.,0.,offset)
        },
        FacingDirection::UpRight => {
            Vec3::new(-offset,0.,offset)
        },
        FacingDirection::Right => {
            Vec3::new(-offset,0.,0.)
        },
        FacingDirection::DownRight => {
            Vec3::new(-offset,0.,-offset)
        },
        FacingDirection::Down => {
            Vec3::new(0.,0.,-offset)
        },
        FacingDirection::DownLeft => {
            Vec3::new(offset,0.,-offset)
        },
        FacingDirection::Left => {
            Vec3::new(offset,0.,0.)
        },
    }
}
