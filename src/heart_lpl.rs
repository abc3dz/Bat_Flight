use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use rand::Rng;

use crate::{GameState, LevelState};

const HEART_SPAWN_X: f32 = 10.0;
const HEART_SPAWN_SECS: f32 = 2.0;
const HEART_SPEED: f32 = 4.0;
const HEART_DESPAWN_X: f32 = -10.0;

#[derive(Component)]
pub struct Heart;

#[derive(Resource)]
struct HeartSpawnTimer(Timer);

pub struct HeartPlugin;

impl Plugin for HeartPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HeartSpawnTimer(Timer::from_seconds(
                HEART_SPAWN_SECS,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (
                    spawn_heart,
                    move_hearts,
                    rotate_hearts,
                    despawn_hearts,
                )
                .run_if(in_state(GameState::Playing))
                .run_if(heart_levels)
        );
    }
}

fn spawn_heart(
    time: Res<Time>,
    mut timer: ResMut<HeartSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rand::rng();

    let y = rng.random_range(-4.0..=4.0);

    commands.spawn((
        Heart,
        SceneRoot(asset_server.load("models/heartlowpoly.glb#Scene0")),
        Transform::from_xyz(HEART_SPAWN_X, y, 0.0),
        GlobalTransform::default(),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
    ));
    //println!("Spawned a heart at y={y}");
}

fn move_hearts(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Heart>>,
) {
    for mut transform in &mut query {
        transform.translation.x -= HEART_SPEED * time.delta_secs();
    }
}
fn rotate_hearts(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Heart>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(3.0 * time.delta_secs());
    }
}
fn despawn_hearts(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Heart>>,
) {
    for (entity, transform) in &query {
        if transform.translation.x < HEART_DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}
fn heart_levels(
    level_state: Res<State<LevelState>>,
) -> bool {
    matches!(
        level_state.get(),
        LevelState::Level2 | LevelState::Level3
    )
}