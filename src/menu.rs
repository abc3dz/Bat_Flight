use bevy::prelude::*;
use crate::GameState;

#[derive(Component)]
struct MenuUI;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        MenuUI,
    )).with_children(|parent| {

        parent.spawn((
            Text::new("BAT FLIGHT"),
            TextFont {
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.8, 0.2)),
        ));

        parent.spawn((
            Text::new(
                "Press SPACE or CLICK to Start\n\n\
                 SPACE / CLICK = Flap\n\
                 S = Shoot Projectile"
            ),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

fn menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse:    Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let start_game = keys.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left);

    if start_game {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}