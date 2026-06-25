use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use rand::Rng;

use crate::{GameState, LevelState, ScreenShake};
use crate::score::Score;
use crate::bat_lpl::{Bat, BatAnimationToPlay, BatProjectile, HurtTimer};
use crate::heart_lpl::HeartsUi;

#[derive(Component)]
pub struct OwlMinion;

#[derive(Resource)]
struct OwlSpawnTimer(Timer);

#[derive(Resource)]
pub struct OwlSettings {
    pub spawn_x: f32,
    pub spawn_sec: f32,
    pub speed: f32,
    pub despawn_x: f32,
}

pub struct OwlMinionPlugin;

impl Plugin for OwlMinionPlugin {
    fn build(&self, app: &mut App) {
        let owl_settings = OwlSettings {
            spawn_x: 10.0,
            spawn_sec: 3.0,
            speed: 9.0,
            despawn_x: -10.0,
        };
        let spawn_sec = owl_settings.spawn_sec;
        app
            .insert_resource(owl_settings)
            .insert_resource(OwlSpawnTimer(Timer::from_seconds(
                spawn_sec,
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
    settings: Res<OwlSettings>
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
        Transform::from_xyz(settings.spawn_x, y, 0.0)
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
    settings: Res<OwlSettings>
) {
    for mut transform in &mut query {
        transform.translation.x -= settings.speed * time.delta_secs();
    }
}

fn despawn_owls(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<OwlMinion>>,
    settings: Res<OwlSettings>
) {
    for (entity, transform) in &query {
        if transform.translation.x < settings.despawn_x {
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
    projectile_query: Query<(Entity, &Transform),With<BatProjectile>>,
    mut shake: ResMut<ScreenShake>,
    mut morph_query: Query<&mut MorphWeights>,
    mut hurt: ResMut<HurtTimer>,
){
    let Ok(bat_t) = bat_query.single() else { return };
    for (entity, owl_transform) in &owl_query {
        let distance = bat_t
            .translation
            .distance(owl_transform.translation);
        if distance < 1.0 {
            shake.timer = 0.3;
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
                asset_server.load("sounds/219816__saraonsins__monkey-scream.ogg"),
            ));
            commands.spawn(AudioPlayer::new(
            asset_server.load("sounds/owl_atk.ogg"),
            ));
            for mut weights in &mut morph_query {
                weights.weights_mut()[0] = 1.0;
            }

            hurt.timer = Some(
                Timer::from_seconds(
                    0.2,
                    TimerMode::Once,
                )
            );
        }
        for (projectile_entity,projectile_transform) in &projectile_query{
            let distance =
                projectile_transform
                .translation
                .distance(
                    owl_transform.translation
                );

            if distance < 1.5 {

                commands.entity(projectile_entity).despawn();
                commands.entity(entity).despawn();
            }
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
