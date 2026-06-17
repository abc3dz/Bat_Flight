use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use rand::Rng;

use crate::GameState;
use crate::bat_lpl::Bat;
use crate::score::Score;

const COIN_SPAWN_X: f32 = 10.0;
const COIN_SPAWN_SECS: f32 = 2.0;
const COIN_SPEED: f32 = 4.0;
const COIN_DESPAWN_X: f32 = -10.0;

#[derive(Component)]
pub struct Coin;

#[derive(Resource)]
struct CoinSpawnTimer(Timer);

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CoinSpawnTimer(Timer::from_seconds(
                COIN_SPAWN_SECS,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (
                    spawn_coin,
                    move_coins,
                    rotate_coins,
                    despawn_coins,
                    check_collision,
                )
                .run_if(in_state(GameState::Playing))
        );
    }
}

fn spawn_coin(
    time: Res<Time>,
    mut timer: ResMut<CoinSpawnTimer>,
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
        Coin,
        SceneRoot(asset_server.load("models/coinlowpoly.glb#Scene0")),
        Transform::from_xyz(COIN_SPAWN_X, y, 0.0),
        GlobalTransform::default(),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
    ));
}

fn move_coins(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Coin>>,
) {
    for mut transform in &mut query {
        transform.translation.x -= COIN_SPEED * time.delta_secs();
    }
}
fn rotate_coins(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Coin>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(3.0 * time.delta_secs());
    }
}
fn despawn_coins(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Coin>>,
) {
    for (entity, transform) in &query {
        if transform.translation.x < COIN_DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}
fn check_collision(
    bat_query: Query<&Transform, With<Bat>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let Ok(bat_t) = bat_query.single() else { return };

    for (entity, coin_transform) in &coin_query {
        let distance = bat_t
            .translation
            .distance(coin_transform.translation);

        if distance < 1.0 {
            if coin_transform.translation.y < -1.0 {
                score.value += 5;
            }else{
                score.value += 1;
            }
            commands.entity(entity).despawn();
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/score.ogg"),
            ));
        }
    }
}