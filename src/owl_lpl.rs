use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use rand::Rng;

use crate::{GameState, LevelState};
use crate::score::Score;
use crate::bat_lpl::{Bat, BatAnimationToPlay};
use crate::heart_lpl::HeartsUi;

const OWL_SPAWN_X: f32 = 10.0;
const OWL_SPAWN_SECS: f32 = 3.0;
const OWL_SPEED: f32 = 9.0;
const OWL_DESPAWN_X: f32 = -10.0;

#[derive(Component)]
pub struct OwlMinion;

#[derive(Resource)]
struct OwlSpawnTimer(Timer);

pub struct OwlMinionPlugin;

impl Plugin for OwlMinionPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(OwlSpawnTimer(Timer::from_seconds(
                OWL_SPAWN_SECS,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (
                    check_collision,
                    spawn_owl_minion,
                    owl_minion_move,
                    despawn_owls,
                )
                .run_if(in_state(GameState::Playing))
                .run_if(owl_levels)
        );
    }
}

fn spawn_owl_minion(
    time: Res<Time>,
    mut timer: ResMut<OwlSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rand::rng();

    let y = rng.random_range(-4.0..=4.0);

    let clip = asset_server.load("models/owllowpoly.glb#Animation1");
    let mut graph = AnimationGraph::new();
    let index = graph.add_clip(clip, 1.0, graph.root);
    let graph_handle = graphs.add(graph);
    commands.insert_resource(BatAnimationToPlay {
        graph: graph_handle,
        index,
    });

    commands.spawn((
        OwlMinion,
        SceneRoot(asset_server.load("models/owllowpoly.glb#Scene0")),
        Transform::from_xyz(OWL_SPAWN_X, y, 0.0)
        .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
        GlobalTransform::default(),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
        // AudioPlayer::new(
        // asset_server.load("sounds/owl_ap.ogg")
        // ),
        // PlaybackSettings::LOOP.with_volume(Volume::Linear(0.1)),
    ));
}

fn owl_minion_move(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<OwlMinion>>,
) {
    for mut transform in &mut query {
        transform.translation.x -= OWL_SPEED * time.delta_secs();
    }
}

fn despawn_owls(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<OwlMinion>>,
) {
    for (entity, transform) in &query {
        if transform.translation.x < OWL_DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collision(
    owl_query: Query<(Entity, &Transform), With<OwlMinion>>,
    bat_query: Query<&Transform, With<Bat>>,
    heartsui_query: Query<Entity, With<HeartsUi>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    mut next: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
){
    let Ok(bat_t) = bat_query.single() else { return };
    for (entity, owl_transform) in &owl_query {
        let distance = bat_t
            .translation
            .distance(owl_transform.translation);
        if distance < 1.0 {
            score.owl += 1;
            if score.heart <= 1 {
                score.heart = 3;
                next.set(GameState::GameOver);
            }else{
                score.coin -= 1;
                score.heart -= 1;
            }
            commands.entity(entity).despawn();
            
            for heart_entity in heartsui_query {
                commands.entity(heart_entity).despawn();
            }
            commands.spawn(AudioPlayer::new(
            asset_server.load("sounds/owl_atk.ogg"),
            ));
        }
    }
}
pub fn owl_levels(
    level_state: Res<State<LevelState>>,
) -> bool {
    matches!(
        level_state.get(),
        LevelState::Level4 | LevelState::Level5
    )
}