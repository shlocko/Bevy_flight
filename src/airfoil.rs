use bevy::prelude::*;
use crate::physics::{step_physics, Body};

#[derive(Component)]
pub struct Airfoil{
    pub cl: f32,
    pub max_thrust: f32,
    pub cur_thrust: f32,
    pub cd: f32,
    pub ci: f32,
}

pub enum ControlType{
    Pitch,
    Roll,
    Yaw,
    Flap
}

#[derive(Component)]
pub struct AeroSurface{
    offset: Vec3,
    rotation: Quat,
    lift_slope: f32,
    friction: f32,
    zero_lift_aoa: f32,
    stall_high: f32,
    stall_low: f32,
    chord: f32,
    flap_fraction: f32,
    span: f32,
    aspect_ratio: f32,
    auto_aspect: bool,
    is_control_surface: bool,
    control_type: ControlType,
}

pub struct AirfoilPlugin;

impl Plugin for AirfoilPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, (apply_thrust.before(step_physics), apply_lift.before(step_physics), apply_drag.before(step_physics)));
    }
}

fn apply_thrust(
    mut query: Query<(&mut Body, &mut Airfoil, &mut Transform)>
               ){
    for (mut body, airfoil, mut transform) in &mut query{
        let mut force = Vec3{z: -airfoil.max_thrust, ..default()};
        force = transform.rotation.mul_vec3(force);
        body.force += force;
    }
}
fn apply_drag(
    mut query: Query<(&mut Body, &mut Airfoil, &mut Transform)>
             ){
    for (mut body, airfoil, mut transform) in &mut query{
        let mut drag_x = 0.5 * (body.velocity.x.powf(2.0)) * 0.2;
        if body.velocity.x > 1.0{
            drag_x *= -1.0;
        }
        let mut drag_y = 0.5 * (body.velocity.y.powf(2.0)) * 0.2;
        if body.velocity.y > 1.0{
            drag_y *= -1.0;
        }
        let mut drag_z = 0.5 * (body.velocity.z.powf(2.0)) * 0.2;
        if body.velocity.z > 1.0{
            drag_z *= -1.0;
        }
        let drag = Vec3{x: drag_x, y: drag_y, z: drag_z};
        body.force += drag;
        //println!("{}", drag);
    }
}
fn apply_lift(
    mut query: Query<(&mut Body, &mut Airfoil, &Transform)>
             ){
    for (mut body, airfoil, transform) in &mut query{
        let mass = body.mass;
        body.force += Vec3{y: 9.81*mass, ..default()};
    }
}
