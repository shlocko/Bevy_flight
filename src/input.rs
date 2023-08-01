use bevy::prelude::*;

use crate::setup::Player;

pub struct InputPlugin;

impl Plugin for InputPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, (handle_input));
    }
}

fn handle_input(
    key: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform, With<Player>>
               ){
    for mut transform in &mut transforms{
        if key.pressed(KeyCode::W){
            transform.rotate(Quat::from_rotation_y(-0.01));
        }
        if key.pressed(KeyCode::M){
            transform.rotate(Quat::from_rotation_y(0.01));
        }
    }
}
