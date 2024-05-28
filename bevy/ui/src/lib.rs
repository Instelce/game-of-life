mod button;

use bevy::prelude::*;
use button::ButtonPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ButtonPlugin);
        app.add_systems(Startup, spawn_ui);
    }
}


#[derive(Component)]
pub struct RootNode;


fn spawn_ui(
    mut commands: Commands
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            // background_color: Color::GRAY.into(),
            ..default()
        },
        RootNode,
        Name::new("UI Root")
    )).with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font_size: 16.,
                        color: Color::BLACK,
                        ..default()
                    }
                )
            );
        });
    });
}