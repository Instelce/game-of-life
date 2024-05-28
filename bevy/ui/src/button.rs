
use bevy::prelude::*;

use crate::RootNode;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Buttons(Vec::new()))
            .insert_resource(ButtonCount(0))
            .add_systems(Update, (create_buttons, buttons_system));
    }
}


pub struct ButtonComponent {
    pub button_bundle: ButtonBundle,
    pub text_bundle: TextBundle,
    pub callback: fn(),
    pub background_color: BackgroundColor,
    pub background_color_hovered: BackgroundColor,
    pub background_color_pressed: BackgroundColor,
    pub text_color: Color,
    pub text_color_hovered: Color,
    pub text_color_pressed: Color,
}

#[derive(Resource)]
pub struct Buttons(pub Vec<ButtonComponent>);

#[derive(Resource)]
struct ButtonCount(pub usize);

pub fn create_buttons(
    mut commands: Commands,
    buttons: Res<Buttons>,
    mut button_count: ResMut<ButtonCount>,

    mut root_node: Query<Entity, With<RootNode>>,
) {
    if button_count.0 == buttons.0.len() {
        return;
    }

    let root_node = root_node.single();
    for button in buttons.0.iter() {
        commands.entity(root_node).with_children(|children| {
            let mut button_entity = children.spawn(button.button_bundle.clone());
            button_entity.with_children(|children| {
                // children.spawn();
            });
        });

        button_count.0 += 1;
    }
}

pub fn buttons_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>)
    >,
    mut text_query: Query<&mut Text>
) {
    for (interaction, mut background_color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Hovered => {
                text.sections[0].style = TextStyle {
                    color: Color::WHITE,
                    ..default()
                };
                *background_color = Color::BLACK.into();
            },
            Interaction::Pressed => {

                // actions of buttons
                

            },
            Interaction::None => {
                text.sections[0].style = TextStyle {
                    color: Color::BLACK,
                    ..default()
                };
                *background_color = Color::WHITE.into();
            }
        }
    }
}

