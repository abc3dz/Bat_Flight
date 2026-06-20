use bevy::prelude::*;

use crate::LevelState;
use crate::bat_lpl::Bat;
use crate::coin_lpl::Coin;
use crate::owl_lpl::Owl;
use crate::pillar_lpl::Pillar;
use crate::gear_lpl::Gear;
use crate::score::Score;
use crate::heart_lpl::{Heart, HeartsUi};


#[derive(Component)]
pub struct LevelEnd;

pub struct LevelEndPlugin;

impl Plugin for LevelEndPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            OnEnter(LevelState::LevelEnd),
            (
                spawn_ending_text,
                cleanup_game,
            )
        );
    }
}

fn spawn_ending_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let summary = format!(
        "Thank you for playing!!\n\n\
        Coins Collected: {}\n\
        Hearts Collected: {}\n\
        Pillars Hit: {}\n\
        Gears Hit: {}\n\
        Owls Hit: {}",
        score.coin,
        score.heart,
        score.pillar,
        score.gear,
        score.owl,
    );
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
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
    });

    commands.spawn(AudioPlayer::new(
    asset_server.load("sounds/ending.ogg"),
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
    owl_query: Query<Entity, With<Owl>>
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
}