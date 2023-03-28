use rand::Rng;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_entities)
        .add_system(move_circle)
        .run();
}
#[derive(Component)]
struct Entity;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    }).insert(Entity);

}

fn move_circle(time: Res<Time>, mut query: Query<&mut Transform, With<Entity>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 10.0 * time.delta_seconds();
    }
}

fn spawn_entities(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let amount: i32 = 10;
    for _i in 0..amount {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(rand::thread_rng().gen_range(0..100) as f32, 0., 0.)),
            ..default()
        }).insert(Entity);
    }
}