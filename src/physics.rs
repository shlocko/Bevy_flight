use bevy::prelude::*;

#[derive(Component)]
pub struct Body{
    pub mass: f32,
    pub velocity: Vec3,
    pub force: Vec3
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, (step_physics));
    }
}

fn step_physics(
    time: Res<Time>,
    mut bodies: Query<(&mut Transform, &mut Body)>,
               ){
    let gravity = Vec3{y: -9.81, ..default()};
    for (mut transform, mut body) in &mut bodies {
        let mass = body.mass;
        body.force += gravity * mass;
        let force = body.force;
        body.velocity += force / mass;
        transform.translation += body.velocity * time.delta_seconds();

        body.force = Vec3::ZERO;
        println!("{}", transform.translation);

    }
}
