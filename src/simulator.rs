use pix_engine::prelude::Engine;

use crate::{engine::SimEngine, maze::Maze, COLS, ROWS};

const RATIO_VIS_MM: i32 = 4;

const CELL_SIZE_MM: i32 = 180;
const WALL_WIDTH_MM: i32 = 12;

pub const CELL_SIZE_VIS: i32 = CELL_SIZE_MM / RATIO_VIS_MM;
pub const WALL_WIDTH_VIS: i32 = WALL_WIDTH_MM / RATIO_VIS_MM;
pub const WALL_LENGTH_VIS: i32 = CELL_SIZE_VIS - WALL_WIDTH_VIS;

pub const APP_HEIGHT: u32 = CELL_SIZE_VIS as u32 * ROWS as u32 + WALL_WIDTH_VIS as u32;
pub const APP_WIDTH: u32 = CELL_SIZE_VIS as u32 * COLS as u32 + WALL_WIDTH_VIS as u32;

pub struct MazeSimulator<const R: usize, const C: usize>;

impl<const R: usize, const C: usize> MazeSimulator<R, C> {
    pub fn run(maze: Maze<R, C>) -> anyhow::Result<()> {
        let mut engine = SimEngine::new(maze);

        let mut pix_engine = Engine::builder()
            .dimensions(APP_WIDTH + 1, APP_HEIGHT + 1)
            .title("Maze Simulator")
            .target_frame_rate(60)
            .build()?;

        pix_engine.run(&mut engine)
    }
}
