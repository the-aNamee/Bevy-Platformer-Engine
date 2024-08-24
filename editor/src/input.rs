use bevy::math::vec2;
use bevy::prelude::*;
use bevy::input::gamepad::{GamepadConnection, GamepadEvent};
use crate::tilemap::TILE_SIZE;
use crate::ui::{MenuRes, CurrentMenu};


const CURSOR_SPEED: f32 = 300.0;
const CURSOR_SIZE: Vec2 = vec2(64.0, 64.0);
const CURSOR_ZED_INDEX: f32 = 100.0;

#[derive(Resource)]
pub struct MyGamepad(Gamepad);

#[derive(Component)]
pub struct Cursor;

pub fn setup_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Cursor
    let cursor_texture: Handle<Image> = asset_server.load("cursor.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec2(0.0, 0.0).extend(CURSOR_ZED_INDEX),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(CURSOR_SIZE),
                ..default()
            },
            texture: cursor_texture,
            ..default()
        },
        Cursor
    ));
}

pub fn cursor_input_system(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    
    mut cursors: Query<&mut Transform, With<Cursor>>,
    mut menu_res: ResMut<MenuRes>,
    time: Res<Time<Fixed>>
) {
    if menu_res.current_menu != CurrentMenu::None {
        // Not dealing with cursor atm.
        return;
    }

    let Some(&MyGamepad(gamepad)) = my_gamepad.as_deref() else {
        // No gamepad is connected.
        return;
    };

    // The joysticks are represented using a separate axis for X and Y.
    let axis_lx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickX
    };
    let axis_ly = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickY
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // Combine X and Y into one vector.
        let left_stick = Vec2::new(x, y);

        let mut cursor_transform = cursors.single_mut();
        
        if left_stick != Vec2::ZERO {
            cursor_transform.translation += (left_stick * time.delta_seconds() * CURSOR_SPEED).extend(0.0);
        } else {
            let tile_size_into = TILE_SIZE.into();
            cursor_transform.translation = tile_snap(cursor_transform.translation.xy(), tile_size_into, tile_size_into / 2.0).extend(cursor_transform.translation.z);
        }
    }

    // In a real game, the buttons would be configurable, but here we hardcode them.
    let place_button = GamepadButton {
        gamepad, button_type: GamepadButtonType::East
    };

    if buttons.pressed(place_button) {
        println!("Place")
    }

    let enter_ui_button = GamepadButton::new(gamepad, GamepadButtonType::DPadUp);
    if buttons.pressed(enter_ui_button) {
        menu_res.set_to_sidebars();
    }
}

fn tile_snap(to_snap: Vec2, snap_value: Vec2, offset: Vec2) -> Vec2 {
    let before_round = (to_snap - offset) / snap_value;
    let rounded = before_round.round();
    let after_round = rounded * snap_value + offset;
    return after_round;
}

pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    ev_conn.gamepad, info.name,
                );
                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
                // if it's the one we previously used for the player, remove it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == ev_conn.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}
