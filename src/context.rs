use anyhow::Result;
use pix_engine::prelude::*;

use crate::{
    engine::Render,
    maze::{Cell, CellState, Maze},
    simulator::{CELL_SIZE_VIS, WALL_WIDTH_VIS},
};

pub struct RunnerContext<const R: usize, const C: usize> {
    maze: Maze<R, C>,
    values: [[Option<i32>; R]; C],
}

impl<const R: usize, const C: usize> RunnerContext<R, C> {
    pub fn new() -> Self {
        Self {
            maze: Maze::new(),
            values: [[None; R]; C],
        }
    }

    pub fn clear_cell(&mut self, cell: Cell<R, C>) {
        self.values[cell.x][cell.y] = None;
        self.maze.update_cell_state(cell, CellState::all(), false);
    }

    pub fn set_cell_state(&mut self, cell: Cell<R, C>, cell_state: CellState) {
        self.maze.update_cell_state(cell, cell_state, true);
    }

    pub fn set_cell_value(&mut self, cell: Cell<R, C>, value: i32) {
        self.values[cell.x][cell.y] = Some(value);
    }
}

impl<const R: usize, const C: usize> Render for RunnerContext<R, C> {
    fn draw<T>(&self, s: &mut PixState, primary_color: T, secondary_color: T) -> Result<()>
    where
        T: Into<Option<Color>> + std::marker::Copy,
    {
        self.maze.draw(s, primary_color, secondary_color)?;

        s.stroke(None);
        s.fill(Color::DIM_GRAY);
        s.font_size(10)?;
        s.font_family(Font::NOTO)?;

        for y in 0..R as usize {
            for x in 0..C as usize {
                match self.values[x][y] {
                    Some(v) => {
                        s.set_cursor_pos([
                            x as i32 * CELL_SIZE_VIS + WALL_WIDTH_VIS + 2,
                            R as i32 * CELL_SIZE_VIS - y as i32 * CELL_SIZE_VIS - CELL_SIZE_VIS + 2,
                        ]);

                        s.text(format!("{}", v))?;
                    }
                    None => {}
                }
            }
        }

        s.font_family(Font::EMULOGIC)?;
        s.font_size(12)
    }
}
