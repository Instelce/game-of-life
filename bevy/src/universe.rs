use std::time::Duration;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::system::{Commands, Res, ResMut, Resource},
    reflect::List,
    time::{Time, Timer, TimerMode},
};
use rand::Rng;

use crate::cell::{Cell, CellStatus};

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Universe::init(100, 100));
        app.add_systems(Startup, setup_timer);
        app.add_systems(Update, tick);
    }
}

#[derive(Resource)]
pub struct Universe {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<Cell>>,
    pub generation_time: u64,
}

#[derive(Resource)]
pub struct UniverseTimer {
    timer: Timer,
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
            generation_time: 200
        }
    }

    pub fn live_neighbor(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for delta_y in [self.height - 1, 0, 1].iter().copied() {
            for delta_x in [self.width - 1, 0, 1].iter().copied() {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }

                let row = (delta_y + y) % self.height;
                let col = (delta_x + x) % self.width;
                count += self.get_cell(col, row).unwrap().status as u8;
            }
        }
        count
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        self.cells
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }

    pub fn next_generation(&mut self) {
        let mut next_cells: Vec<Vec<Cell>> = Vec::new();

        for y in 0..self.height {
            let mut row_vec = Vec::new();

            for x in 0..self.width {
                match self.get_cell(x, y) {
                    Some(cell) => {
                        let cell_status = &cell.status;
                        let live_neighbor = self.live_neighbor(x, y);

                        let new_cell_status = match (cell_status, live_neighbor) {
                            (CellStatus::Alive, x) if x < 2 => CellStatus::Dead,
                            (CellStatus::Alive, 2) | (CellStatus::Alive, 3) => CellStatus::Alive,
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

        self.cells = next_cells;
    }
}

fn tick(mut universe: ResMut<Universe>, time: Res<Time>, mut universe_timer: ResMut<UniverseTimer>) {
    universe_timer.timer.tick(time.delta());

    if universe_timer.timer.finished() {
        universe.next_generation();
    }
}

fn setup_timer(
    mut commands: Commands,
    universe: Res<Universe>,
) {
    commands.insert_resource(UniverseTimer {
        timer: Timer::new(Duration::from_millis(universe.generation_time), TimerMode::Repeating)
    })
}