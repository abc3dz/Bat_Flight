use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use bevy::audio::Volume;

use crate::{GameState, LevelState, score};
use crate::score::Score;
use crate::bat_lpl::{Bat, AnimationToPlay};
use crate::heart_lpl::{HeartsContainer, HeartsUi};

#[derive(Component)]
pub struct OwlBoss;

#[derive(Component)]
pub struct BossHpBar;

#[derive(Component)]
pub struct BossHpFill;

#[derive(Component)]
pub struct OwlBossHp {
    pub current: u32,
    pub max: u32,
}

pub struct OwlBossPlugin;

impl Plugin for OwlBossPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(LevelState::Level1),spawn_owl_boss,)
            .add_systems(
                Update,
                (
                    check_collision,
                    owl_boss_move,
                )
                //.chain()
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(LevelState::Level1))
        );
    }
}   

fn spawn_owl_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    score: Res<Score>
){
    let clip = asset_server.load("models/owllowpoly.glb#Animation1");
    let mut graph = AnimationGraph::new();
    let index = graph.add_clip(clip, 1.0, graph.root);
    let graph_handle = graphs.add(graph);
    commands.insert_resource(AnimationToPlay {
        graph: graph_handle,
        index,
    });

    commands.spawn((
        OwlBoss,
        SceneRoot(asset_server.load("models/owllowpoly.glb#Scene0")),
        Transform::from_xyz(5.0, 0.0, 0.0)
        .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
        GlobalTransform::default(),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Day)
            .weather(Weather::Sunny)
            .build(),
        AudioPlayer::new(
        asset_server.load("sounds/owl_ap.ogg")
        ),PlaybackSettings::LOOP.with_volume(Volume::Linear(0.1))
    ));

    commands.spawn((
        BossHpBar,
        Node {
            position_type: PositionType::Absolute,
            top: px(20.0),
            left: Val::Percent(25.0),
            width: px(500.0),
            height: px(30.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
    ))
    .with_children(|parent| {
        parent.spawn((
            BossHpFill,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
        ));
    });
}

fn owl_boss_move(
    bat_query: Query<&Transform, (With<Bat>, Without<OwlBoss>)>,
    mut owl_query: Query<&mut Transform, With<OwlBoss>>,
) {
    let Ok(bat_transform) = bat_query.single() else {
        return;
    };

    for mut owl_transform in &mut owl_query {
        owl_transform.translation.y = bat_transform.translation.y;
    }
}

fn check_collision(
    owl_query: Query<(Entity, &Transform), With<OwlBoss>>,
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