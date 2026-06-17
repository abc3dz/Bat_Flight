use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use rand::Rng;

use crate::{GameState, LevelState};
use crate::score::Score;
use crate::bat_lpl::Bat;
use crate::heart_lpl::HeartsUi;

const GEAR_SPAWN_X: f32 = 10.0;
const GEAR_SPAWN_SECS: f32 = 2.0;
const GEAR_SPEED: f32 = 4.0;
const GEAR_DESPAWN_X: f32 = -10.0;

#[derive(Component)]
pub struct Gear;

#[derive(Resource)]
struct GearSpawnTimer(Timer);

pub struct GearPlugin;

impl Plugin for GearPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GearSpawnTimer(Timer::from_seconds(
                GEAR_SPAWN_SECS,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (
                    check_collision,
                    spawn_gear,
                    move_gears,
                    rotate_gears,
                    despawn_gears,
                )
                //.chain()
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(LevelState::Level3))
        );
    }
}

fn spawn_gear(
    time: Res<Time>,
    mut timer: ResMut<GearSpawnTimer>,
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
        Gear,
        SceneRoot(asset_server.load("models/gearlowpoly.glb#Scene0")),
        Transform::from_xyz(GEAR_SPAWN_X, y, 0.0),
        GlobalTransform::default(),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
    ));
}

fn move_gears(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Gear>>,
) {
    for mut transform in &mut query {
        transform.translation.x -= GEAR_SPEED * time.delta_secs();
    }
}
fn rotate_gears(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Gear>>,
) {
    for mut transform in &mut query {
        transform.rotate_z(3.0 * time.delta_secs());
    }
}
fn despawn_gears(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Gear>>,
) {
    for (entity, transform) in &query {
        if transform.translation.x < GEAR_DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}
fn check_collision(
    gear_query: Query<(Entity, &Transform), With<Gear>>,
    bat_query: Query<&Transform, With<Bat>>,
    heartsui_query: Query<Entity, With<HeartsUi>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    mut next: ResMut<NextState<GameState>>,
){
    let Ok(bat_t) = bat_query.single() else { return };
    for (entity, gear_transform) in &gear_query {
        let distance = bat_t
            .translation
            .distance(gear_transform.translation);
        if distance < 1.0 {
            println!("heart = {}", score.heart);
            if score.heart <= 1 {
                score.heart = 3;
                next.set(GameState::GameOver);
            }else{
                score.value -= 1;
                score.heart -= 1;
            }
            commands.entity(entity).despawn();
            
            for heart_entity in heartsui_query {
                commands.entity(heart_entity).despawn();
            }
        }
    }
}