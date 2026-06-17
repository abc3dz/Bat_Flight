use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use rand::Rng;

use crate::{GameState, LevelState};
use crate::score::Score;
use crate::bat_lpl::Bat;

const HEART_SPAWN_X: f32 = 10.0;
const HEART_SPAWN_SECS: f32 = 2.0;
const HEART_SPEED: f32 = 4.0;
const HEART_DESPAWN_X: f32 = -10.0;

#[derive(Component)]
pub struct Heart;

#[derive(Component)]
struct HeartsContainer;

#[derive(Component)]
pub struct HeartsUi;

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
                    check_collision,
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
    score: Res<Score>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }
    if score.heart>=6 {
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

    
    commands.spawn((
        HeartsContainer,
        Node {
            position_type: PositionType::Absolute,
            top: px(10.0),
            left: px(10.0),
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {
        for _ in 0..score.heart {
            parent.spawn((
                HeartsUi,
                ImageNode::new(
                    asset_server.load("images/heart.png")
                ),
                Node {
                    width: px(50.0),
                    height: px(50.0),
                    ..default()
                }
            ));
        }
    });
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
fn check_collision(
    mut commands: Commands,
    bat_query: Query<&Transform, With<Bat>>,
    heart_query_trans: Query<(Entity, &Transform), With<Heart>>,
    container_query: Query<Entity, With<HeartsContainer>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
){
    let Ok(bat_t) = bat_query.single() else { return };
    for (entity, heart_trans) in &heart_query_trans {
        let distance = bat_t
            .translation
            .distance(heart_trans.translation);

        if distance < 1.0 {
            score.heart += 1;
            commands.entity(entity).despawn();

            if let Ok(container) = container_query.single() {
                commands.entity(container).with_children(|parent| {
                    parent.spawn((
                        HeartsUi,
                        ImageNode::new(asset_server.load("images/heart.png")),
                        Node {
                            width: px(50.0),
                            height: px(50.0),
                            ..default()
                        }
                    ));
                });
                
            }
            commands.spawn(AudioPlayer::new(
            asset_server.load("sounds/heart.ogg"),
            ));
        }
    }
}