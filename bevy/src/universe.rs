use bevy::{
    app::{Plugin, Update},
    ecs::system::{ResMut, Resource}, reflect::List,
};
use rand::Rng;

use crate::cell::{Cell, CellStatus};

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Universe::init(100, 100));
        app.add_systems(Update, tick);
    }
}

#[derive(Resource)]
pub struct Universe {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<Cell>>,
}

impl Universe {
    fn init(width: u32, height: u32) -> Universe {
        let mut random = rand::thread_rng();

        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for y in 0..height {
            let mut row_vec = Vec::<Cell>::new();

            for x in 0..width {
                let cell = Cell::new(
                    x as f32,
                    y as f32,
                    if random.gen_bool(1. / 2.) {
                        CellStatus::Alive
                    } else {
                        CellStatus::Dead
                    },
                );

                // random.gen_bool(1. / 2.)
                // (x * y) % 2 == 0 || (x * y) % 7 == 0

                row_vec.push(cell);
            }

            cells.push(row_vec);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn live_neighbor(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().copied() {
            for delta_col in [self.width - 1, 0, 1].iter().copied() {
                let row = (delta_row + row) % self.width;
                let col = (delta_col + col) % self.width;
                count += match self.get_cell(col, row) {
                    Some(cell) => cell.status as u8,
                    None => 0
                };
            }
        }
        count
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        self.cells.get(y as usize).and_then(|row| row.get(x as usize))
    }
}

fn tick(mut universe: ResMut<Universe>) {
    let mut next_cells: Vec<Vec<Cell>> = Vec::new();

    for y in 0..universe.height {
        let mut row_vec = Vec::new();

        for x in 0..universe.width {

            match universe.get_cell(x, y) {
                Some(cell) => {
                    let cell_status = &cell.status;
                    let live_neighbor = universe.live_neighbor(x, y);

                    let new_cell_status = match (cell_status, live_neighbor) {
                        (CellStatus::Alive, x) if x < 2 => CellStatus::Dead,
                        (CellStatus::Alive, 2) => CellStatus::Alive,
                        (CellStatus::Alive, x) if x > 3 => CellStatus::Dead,
                        (CellStatus::Dead, 3) => CellStatus::Alive,
                        (other, _) => other.clone(),
                    };

                    let new_cell = Cell::new(x as f32, y as f32, new_cell_status);

                    row_vec.push(new_cell);
                }
                None => {}
            };
        }
        next_cells.push(row_vec);
    }

    universe.cells = next_cells;
}
