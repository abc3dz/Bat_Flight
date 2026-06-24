use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;

use crate::GameState;
use crate::score::{Score, CoinUi};

const GRAVITY:    f32 = -12.0;
const FLAP_FORCE: f32 =  6.0;

#[derive(Component)]
pub struct Bat {
    pub velocity_y: f32,
}

#[derive(Component)]
pub struct BatProjectile;

#[derive(Resource)]
pub struct BatAnimationToPlay {
    pub graph: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
}

#[derive(Resource, Default)]
pub struct CoinShake {
    pub timer: f32,
}

pub struct BatPlugin;

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CoinShake::default())
        .add_plugins(WindWakerShaderPlugin::default())
        //.add_systems(Startup, spawn_bat)
        .add_systems(Update, (
            play_animation_when_ready,
            bat_input,
            bat_physics,
            move_projectiles,
            shake_coin_ui,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), cleanup_bat)
        .add_systems(OnEnter(GameState::Playing), spawn_bat);
    }
}

fn spawn_bat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
        let clip = asset_server.load("models/batlowpoly.glb#Animation0");
        let mut graph = AnimationGraph::new();
        let index = graph.add_clip(clip, 1.0, graph.root);
        let graph_handle = graphs.add(graph);
        commands.insert_resource(BatAnimationToPlay {
            graph: graph_handle,
            index,
        });

    commands.spawn((
            SceneRoot(asset_server.load("models/batlowpoly.glb#Scene0")),
            Transform::from_xyz(-3.0, 3.0, 0.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            WindWakerShaderBuilder::default()
                .time_of_day(TimeOfDay::Day)
                .weather(Weather::Sunny)
                .build(),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            Bat { velocity_y: 5.0 }
        ));
}

fn bat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse:    Res<ButtonInput<MouseButton>>,
    touches:  Res<Touches>,
    mut query: Query<&mut Bat>,
    bat_transform: Query<&Transform, With<Bat>>,
    mut commands: Commands,        
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut coin: ResMut<Score>,
    mut shake: ResMut<CoinShake>,
) {
    let flapped = keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();

    if flapped {
        for mut bat in &mut query {
            bat.velocity_y = FLAP_FORCE;
        }
        
        commands.spawn(AudioPlayer::new(
            asset_server.load("sounds/fly.ogg"),
        ));
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        shake.timer = 0.3;
        if coin.coin <= 0{
            return;
        }else if coin.coin >= 2{
            coin.coin = coin.coin.saturating_sub(2);
        }else {
            return;
        }
        if let Ok(bat_transform) = bat_transform.single() {
            
           commands.spawn((
                BatProjectile,
                Mesh3d(meshes.add(Sphere::new(0.3))),
                MeshMaterial3d(materials.add(
                    StandardMaterial {
                        base_color: Color::srgb(0.4, 0.0, 0.6),
                        emissive: LinearRgba::rgb(1.0, 0.0, 1.0),
                        ..default()
                    }
                )),
                Transform::from_translation(
                    bat_transform.translation
                ),
            ));
        }
    }
}

fn move_projectiles(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<BatProjectile>>,
) {
    for mut transform in &mut query {
        transform.translation.x +=
            8.0 * time.delta_secs();
    }
}

fn bat_physics(
    time: Res<Time>,
    mut query: Query<(&mut Bat, &mut Transform)>,
    mut next: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut score: ResMut<Score>
) {
    for (mut bat, mut transform) in &mut query {
        bat.velocity_y += GRAVITY * time.delta_secs();
        transform.translation.y += bat.velocity_y * time.delta_secs();
        if transform.translation.y < -5.0 {
            score.game_over += 1;
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/game_over.ogg"),
            ));
            next.set(GameState::GameOver);
        }
    }
}

pub fn play_animation_when_ready(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
    anim: Res<BatAnimationToPlay>,
) {
    for (entity, mut player) in &mut players {
        commands.entity(entity).insert(AnimationGraphHandle(anim.graph.clone()));
        player.play(anim.index).repeat();
    }
}

fn cleanup_bat(
    mut commands: Commands,
    query: Query<Entity, With<Bat>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn shake_coin_ui(
    time: Res<Time>,
    mut shake: ResMut<CoinShake>,
    mut query: Query<&mut Node, With<CoinUi>>,
) {
    if shake.timer <= 0.0 {
        return;
    }

    shake.timer -= time.delta_secs();

    let offset = (time.elapsed_secs() * 80.0).sin() * 6.0;

    for mut node in &mut query {
        node.margin = UiRect::left(px(offset));
    }

    if shake.timer <= 0.0 {
        for mut node in &mut query {
            node.margin = UiRect::ZERO;
        }
    }
}