use bevy::{app::{Plugin, Startup}, asset::Handle, core::Name, ecs::{component::Component, reflect::ReflectComponent, system::{Commands, Resource}}, prelude::SpatialBundle, reflect::Reflect, render::color::Color, sprite::ColorMaterial};

// constants
pub const COLOR_ALIVE: Color = Color::WHITE;
pub const COLOR_DEAD: Color = Color::BLACK;


// plugin
pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.register_type::<Cell>();
        // app.add_systems(Startup, spawn_cell_parent);
    }
} 


// ressources
#[derive(Resource)]
pub struct CellDeadColor(pub Handle<ColorMaterial>);

#[derive(Resource)]
pub struct CellAliveColor(pub Handle<ColorMaterial>);


// component
#[derive(Component)]
pub struct CellParent;

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum CellStatus {
    Dead = 0,
    Alive = 1
}

#[derive(Debug, Component, Clone, PartialEq)]
// #[reflect(Component)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub status: CellStatus
}

impl Cell {

    pub fn new(x: f32, y: f32, status: CellStatus) -> Cell {
        Cell {
            x,
            y,
            status
        }
    }

    pub fn change_status(&mut self, status: CellStatus) {
        self.status = status;
    }

    pub fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

}


pub fn spawn_cell_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), CellParent, Name::new("Cell Container")));
}
