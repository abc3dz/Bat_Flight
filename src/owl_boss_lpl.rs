use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use bevy::audio::Volume;

use crate::{GameState, LevelState};
use crate::score::Score;
use crate::bat_lpl::{Bat, AnimationToPlay};

#[derive(Component)]
pub struct OwlBoss{
    pub direction: f32,
}

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
            .add_systems(OnEnter(GameState::Playing),spawn_owl_boss,)
            .add_systems(
                Update,
                (
                    update_boss_hp_bar,
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
        OwlBoss{
            direction: 1.0
        },
        OwlBossHp {
            current: 10,
            max: 10,
        },
        SceneRoot(asset_server.load("models/owllowpoly.glb#Scene0")),
        Transform::from_xyz(5.0, 0.0, 0.0)
        .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
        GlobalTransform::default(),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Night)
            .weather(Weather::Rainy)
            .build(),
        AudioPlayer::new(
        asset_server.load("sounds/owl_ap.ogg")
        ),PlaybackSettings::LOOP.with_volume(Volume::Linear(0.01))
    ));

    commands.spawn((
        BossHpBar,
        Node {
            position_type: PositionType::Absolute,
            top: px(20.0),
            right: px(25.0),
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
            BackgroundColor(Color::srgb(0.55, 0.27, 0.07))
        ));
    });
}

fn owl_boss_move(
    time: Res<Time>,
    mut owl_query: Query<(&mut Transform, &mut OwlBoss)>,
) {
    for (mut transform, mut owl) in &mut owl_query {

        let speed = 3.0;

        transform.translation.y +=
            owl.direction * speed * time.delta_secs();

        if transform.translation.y > 4.0 {
            owl.direction = -1.0;
        }

        if transform.translation.y < -4.0 {
            owl.direction = 1.0;
        }
    }
}

fn update_boss_hp_bar(
    boss_query: Query<&OwlBossHp>,
    mut fill_query: Query<&mut Node, With<BossHpFill>>,
) {
    let Ok(hp) = boss_query.single() else {
        return;
    };

    let percent =
        hp.current as f32 / hp.max as f32 * 100.0;

    let Ok(mut node) = fill_query.single_mut() else {
        return;
    };

    node.width = Val::Percent(percent);
}
