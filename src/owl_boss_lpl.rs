use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;
use bevy::audio::Volume;

use crate::{GameState, LevelState};
use crate::score::Score;
use crate::bat_lpl::BatLaser;

#[derive(Component)]
pub struct OwlBoss{
    pub direction: f32,
}

#[derive(Component)] pub struct OwlBossTag;

#[derive(Resource)]
pub struct OwlBossAnim {
    pub graph: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
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
            .add_systems(OnEnter(LevelState::Level5),spawn_owl_boss,)
            .add_systems(Update, debug_level5)
            .add_systems(
                Update,
                (
                    //spawn_owl_boss,
                    update_boss_hp_bar,
                    play_owl_anim,
                    owl_boss_move,
                    test_hp_owl_boss,
                )
                //.chain()
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(LevelState::Level5))
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
    commands.insert_resource(OwlBossAnim {
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
        .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2))
        .with_scale(Vec3::splat(2.0)),
        GlobalTransform::default(),
        WindWakerShaderBuilder::default()
            .time_of_day(TimeOfDay::Night)
            .weather(Weather::Rainy)
            .build(),
        AudioPlayer::new(
        asset_server.load("sounds/owl_ap.ogg")
        ),PlaybackSettings::LOOP.with_volume(Volume::Linear(0.01)),
        OwlBossTag
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
        BackgroundColor(Color::WHITE),
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

fn play_owl_anim(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    owl_roots: Query<Entity, With<OwlBoss>>,
    anim: Res<OwlBossAnim>,
) {
    for (entity, mut player) in &mut players {
        let mut current = entity;
        let mut is_owl_child = false;

        while let Ok(parent) = parents.get(current) {
            let parent_entity = parent.parent();

            if owl_roots.get(parent_entity).is_ok() {
                is_owl_child = true;
                break;
            }

            current = parent_entity;
        }

        if !is_owl_child {
            continue;
        }

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(anim.graph.clone()));

        player.play(anim.index).repeat();
    }
}

fn debug_level5(
    keys: Res<ButtonInput<KeyCode>>,
    //mut next_level_state: ResMut<NextState<LevelState>>,
    mut score: ResMut<Score>,
) {
    if keys.just_pressed(KeyCode::KeyZ) {
        score.coin += 10;
        //next_level_state.set(LevelState::Level5);
    }
}
fn test_hp_owl_boss(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut boss_query: Query<&mut OwlBossHp>,
) {
    if keyboard.just_pressed(KeyCode::KeyA) {

        let Ok(mut hp) = boss_query.single_mut() else {
            return;
        };

        hp.current = hp.current.saturating_sub(2);

        println!("Boss HP: {}", hp.current);
    }
}

fn check_collision(
    owl_query: Query<(Entity, &Transform), With<OwlBoss>>,
    bat_laser: Query<&Transform, With<BatLaser>>,
    mut owl_boss_hp: ResMut<OwlBossHp>,
    
    commands: Commands,
    mut next: ResMut<NextState<GameState>>,
    //asset_server: Res<AssetServer>,
){
    let Ok(bat_laser) = bat_laser.single() else { return };
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