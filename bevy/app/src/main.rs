mod camera;
mod cell;
mod universe;

use bevy::{input::common_conditions::input_toggle_active, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use cell::*;
use universe::*;
use ui::UiPlugin;

// types
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

const CELL_SIZE: f32 = 10.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            CameraPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            UiPlugin,
            UniversePlugin,
            CellPlugin
        ))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, (spawn_cell_parent, setup).chain())
        .add_systems(Update, draw_cells)
        .run();
}

fn setup(
    mut commands: Commands,
    universe: Res<Universe>,
    cell_parent: Query<Entity, With<CellParent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // setup cells colors
    let alive_color_handle = materials.add(COLOR_ALIVE);
    let dead_color_handle = materials.add(COLOR_DEAD);

    commands.insert_resource(CellAliveColor(alive_color_handle.clone()));
    commands.insert_resource(CellDeadColor(dead_color_handle.clone()));

    // create cells
    let cell_parent = cell_parent.single();

    for row in &universe.cells {
        for cell in row {
            // add cell inside cell_parent
            commands.entity(cell_parent).with_children(|commands| {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE)).into(),
                        material: materials.add(COLOR_DEAD),
                        transform: Transform::from_xyz(CELL_SIZE * cell.x, -CELL_SIZE * cell.y, 0.),
                        ..default()
                    },
                    cell.clone(),
                    Name::new("Cell"),
                ));
            });
        }
    }
}

fn draw_cells(
    mut commands: Commands,
    mut universe: ResMut<Universe>,
    mut color_materials: Query<(&Cell, &mut Handle<ColorMaterial>), With<Cell>>,
    dead_cell_color: Res<CellDeadColor>,
    alive_cell_color: Res<CellAliveColor>,
) {
    for (cell, mut material) in color_materials.iter_mut() {
        let cell_status = match universe.get_cell(cell.x as u32, cell.y as u32) {
            Some(cell) => cell.status,
            None => CellStatus::Dead,
        };

        if cell_status == CellStatus::Alive {
            *material = alive_cell_color.0.clone();
        } else {
            *material = dead_cell_color.0.clone();
        }
    }
}
