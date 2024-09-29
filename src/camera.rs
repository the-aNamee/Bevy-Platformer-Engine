use bevy::{prelude::*, window::{PrimaryWindow, WindowResized}};

use crate::player::Player;


const ASPECT_RATIO: f32 = 4.0 / 3.0;

pub fn camera_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = player_transform.translation;
        }
    }
}