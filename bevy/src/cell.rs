use bevy::{asset::Handle, ecs::{component::Component, system::Resource}, render::color::Color, sprite::ColorMaterial};

// constants
pub const COLOR_ALIVE: Color = Color::WHITE;
pub const COLOR_DEAD: Color = Color::BLACK;


// ressources
#[derive(Resource)]
pub struct CellDeadColor(pub Handle<ColorMaterial>);

#[derive(Resource)]
pub struct CellAliveColor(pub Handle<ColorMaterial>);


// component
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellStatus {
    Dead = 0,
    Alive = 1
}

#[derive(Debug, Component, Clone, PartialEq)]
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
