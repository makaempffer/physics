use rand::Rng;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::{ PresentMode }};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "I am a window!".into(),
            resolution: (800., 800.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
      }))
        .add_startup_system(setup)
        .add_startup_system(spawn_entities)
        .add_system(update_entities)
        .add_system(apply_gravity)
        .add_system(border_collision)
        .run();
}

#[derive(Component)]
struct Velocity { velocity: Vec3 }
#[derive(Component)]
struct Entity { }

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn border_collision(mut query: Query<(&mut Velocity, &mut Transform), With<Entity>>) {
    for mut transform in query.iter_mut() {
        if transform.1.translation.y <= -400.0 {
            transform.0.velocity.y *= -1.0;
        }
    }
}

fn update_entities(time: Res<Time>, mut query: Query<(&mut Velocity, &mut Transform), With<Entity>>) {
    for mut transform in query.iter_mut() {
        transform.1.translation.y += transform.0.velocity.y * time.delta_seconds();
    } 
}

fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Entity>>) {
    for mut velocity in query.iter_mut() {
        velocity.as_mut().velocity.y -= 9.8 * time.delta_seconds();
    }
}


fn _move_circle(time: Res<Time>, mut query: Query<&mut Transform, With<Entity>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.y > -400.0 {
            transform.translation.y -= 9.8 * time.delta_seconds();
        }
        
    }
}
//rand::thread_rng().gen_range(0..800) as f32

fn spawn_entities(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let amount: i32 = 10;
    for _i in 0..amount {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3 { x: (rand::thread_rng().gen_range(-400..400) as f32), y: (rand::thread_rng().gen_range(-400..400) as f32), z: (0.0) }),
            ..default()
        }).insert((Entity {}, Velocity { velocity: Vec3::ZERO }));
    }
}