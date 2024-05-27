use bevy::{app::{Plugin, Startup, Update}, core::Name, ecs::system::Commands, prelude::default, render::color::Color, ui::{node_bundles::NodeBundle, AlignItems, Style, UiRect, Val}};



pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_ui);
    }
}

fn spawn_ui(
    mut commands: Commands
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Vh(10.),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        },
        Name::new("UI Root")
    ));
}