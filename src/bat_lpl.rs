use bevy::prelude::*;
use bevy_wind_waker_shader::prelude::*;

use crate::GameState;

const GRAVITY:    f32 = -12.0;
const FLAP_FORCE: f32 =  6.0;

#[derive(Component)]
pub struct Bat {
    pub velocity_y: f32,
}

#[derive(Resource)]
pub struct AnimationToPlay {
    pub graph: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
}

pub struct BatPlugin;

impl Plugin for BatPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(WindWakerShaderPlugin::default())
        .add_systems(Startup, spawn_bat)
        .add_systems(Update, (
            play_animation_when_ready,
            bat_input,
            bat_physics,
        ).chain().run_if(in_state(GameState::Playing)));
    }
}

fn spawn_bat
    (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    ) {
        let clip = asset_server.load("models/batlowpoly.glb#Animation0");
        let mut graph = AnimationGraph::new();
        let index = graph.add_clip(clip, 1.0, graph.root);
        let graph_handle = graphs.add(graph);
        commands.insert_resource(AnimationToPlay {
            graph: graph_handle,
            index,
        });

    commands.spawn((
            SceneRoot(asset_server.load("models/batlowpoly.glb#Scene0")),
            Transform::from_xyz(-3.0, 5.0, 0.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            WindWakerShaderBuilder::default()
                .time_of_day(TimeOfDay::Day)
                .weather(Weather::Sunny)
                .build(),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            Bat { velocity_y: 5.0 },
        ));
}

fn bat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse:    Res<ButtonInput<MouseButton>>,
    touches:  Res<Touches>,
    mut query: Query<&mut Bat>,
    mut morph_query: Query<&mut MorphWeights>,
    mut commands: Commands,        
    asset_server: Res<AssetServer>,
) {
    let flapped = keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();

    if flapped {
        for mut bat in &mut query {
            bat.velocity_y = FLAP_FORCE;
        }
        for mut weights in &mut morph_query {
            weights.weights_mut()[0] = 1.0; // index 0 = shape key "fly"
        }
        commands.spawn(AudioPlayer::new(
            asset_server.load("sounds/fly.ogg"),
        ));
    }
}

fn bat_physics(
    time: Res<Time>,
    mut query: Query<(&mut Bat, &mut Transform)>,
    mut next: ResMut<NextState<GameState>>, 
) {
    for (mut bat, mut transform) in &mut query {
        bat.velocity_y += GRAVITY * time.delta_secs();
        transform.translation.y += bat.velocity_y * time.delta_secs();
        if transform.translation.y < -5.0 {
            next.set(GameState::GameOver);
            transform.translation.y = 0.0;
            bat.velocity_y = 2.0;
        }
    }
}

fn play_animation_when_ready(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    anim: Res<AnimationToPlay>,
) {
    for (entity, mut player) in &mut players {
        commands.entity(entity).insert(AnimationGraphHandle(anim.graph.clone()));
        player.play(anim.index).repeat();
    }
}