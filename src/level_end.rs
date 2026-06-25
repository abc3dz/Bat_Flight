use bevy::prelude::*;

use crate::LevelState;
use crate::bat_lpl::Bat;
use crate::coin_lpl::Coin;
use crate::owl_lpl::OwlMinion;
use crate::owl_boss_lpl::{OwlBoss,BossHpBar,BossHpFill};
use crate::pillar_lpl::Pillar;
use crate::gear_lpl::Gear;
use crate::score::Score;
use crate::heart_lpl::{Heart, HeartsUi};
use crate::TimeScore;
use crate::GameState;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct LevelEnd;

pub struct LevelEndPlugin;

impl Plugin for LevelEndPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, debug_level_end)
        .add_systems(
            Update,
            restart_button_system
                .run_if(in_state(LevelState::LevelEnd))
        )
        .add_systems(OnEnter(LevelState::LevelEnd),(spawn_ending_text, cleanup_game,))
        .add_systems(OnExit(LevelState::LevelEnd), cleanup_level_end);
    }
}

fn spawn_ending_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    time_score: Res<TimeScore>,
) {
    let total_seconds = time_score.seconds as u32;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    
    let summary = format!(
        "Thank you for playing!!\n\n\
        Play Time: {:02}:{:02}\n\
        Hearts Collected: {}\n\
        Pillars Hit: {}\n\
        Gears Hit: {}\n\
        Owls Hit: {}\n\
        Game Over: {}",
        minutes,
        seconds,
        score.heart,
        score.pillar,
        score.gear,
        score.owl,
        score.game_over
    );
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        LevelEnd,
    )).with_children(|parent| {
        parent.spawn((
            Text::new(summary),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.7, 0.2)),
        ));
        parent.spawn((
            Button,
            Node {
                width: px(220.0),
                height: px(60.0),
                margin: UiRect::top(px(30.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.8)),
            RestartButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Restart Game"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
    

    commands.spawn(AudioPlayer::new(
    asset_server.load("sounds/25294__freesound__claping2_ses1.ogg"),
    ));
}

fn cleanup_game(
    mut commands: Commands,
    bat_query: Query<Entity, With<Bat>>,
    coin_query: Query<Entity, With<Coin>>,
    pillar_query: Query<Entity, With<Pillar>>,
    gear_query: Query<Entity, With<Gear>>,
    heart_query: Query<Entity, With<Heart>>,
    heartsui_query: Query<Entity, With<HeartsUi>>,
    owl_query: Query<Entity, With<OwlMinion>>,
    owl_boss_query: Query<Entity, With<OwlBoss>>,
    boss_hp_bar_query: Query<Entity, With<BossHpBar>>,
    boss_hp_fill_query: Query<Entity, With<BossHpFill>>,
) {
    for entity in &bat_query {
        commands.entity(entity).despawn();
    }
    for entity in &coin_query {
        commands.entity(entity).despawn();
    }
    for entity in &pillar_query {
        commands.entity(entity).despawn();
    }
    for entity in &gear_query {
        commands.entity(entity).despawn();
    }
    for entity in &heart_query {
        commands.entity(entity).despawn();
    }
    for entity in heartsui_query {
        commands.entity(entity).despawn();
    }
    for entity in owl_query {
        commands.entity(entity).despawn();
    }
    for entity in owl_boss_query {
        commands.entity(entity).despawn();
    }
    for entity in &boss_hp_fill_query {
        commands.entity(entity).despawn();
    }

    for entity in &boss_hp_bar_query {
        commands.entity(entity).despawn();
    }
}

fn debug_level_end(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    if keys.just_pressed(KeyCode::F12) {
        next_level_state.set(LevelState::LevelEnd);
    }
}
fn restart_button_system(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<RestartButton>)
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut score: ResMut<Score>,
    mut time_score: ResMut<TimeScore>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            println!("Restart Clicked!");
            *score = Score::default();
            *time_score = TimeScore::default();

            next_level_state.set(LevelState::Level1);
            next_game_state.set(GameState::Playing);
        }
    }
}
fn cleanup_level_end(
    mut commands: Commands,
    query: Query<Entity, With<LevelEnd>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}