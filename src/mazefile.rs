use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::maze::{Cell, CellState, Maze};

pub struct Mazefile<const R: usize, const C: usize> {
    input: String,
}

impl<const R: usize, const C: usize> Mazefile<R, C> {
    pub fn load(path: PathBuf) -> Result<Self> {
        let mut file = File::open(path).context("Couldn't open mazefile")?;

        let mut input = String::new();
        file.read_to_string(&mut input)
            .context("Couldn't read mazefile")?;

        Ok(Self { input })
    }

    pub fn parse(self) -> Result<Maze<R, C>> {
        let lines: Vec<&str> = self.input.lines().collect();

        if lines.is_empty() {
            bail!("No valid input!");
        }

        let mut maze = Maze::<R, C>::new();

        let mut line_index = 0;
        for y in 0..R {
            let row = R - y - 1;

            for x in 0..C {
                let cell = Cell::new(x, row)?;

                match lines[line_index].as_bytes()[x * 4 + 2] {
                    b'-' => maze.update_cell_state(cell, CellState::NorthWall, true),
                    _ => {}
                }
            }

            line_index += 1;
            for x in 0..C {
                let cell = Cell::new(x, row)?;

                match lines[line_index].as_bytes()[x * 4] {
                    b'|' => maze.update_cell_state(cell, CellState::WestWall, true),
                    _ => {}
                }

                match lines[line_index].as_bytes()[x * 4 + 4] {
                    b'|' => maze.update_cell_state(cell, CellState::EastWall, true),
                    _ => {}
                }

                match lines[line_index].as_bytes()[x * 4 + 2] {
                    b'G' => maze.set_goal_cell(cell)?,
                    b'S' => maze.set_start_cell(cell),
                    _ => {}
                }
            }

            line_index += 1;
            for x in 0..C {
                let cell = Cell::new(x, row)?;

                match lines[line_index].as_bytes()[x * 4 + 2] {
                    b'-' => maze.update_cell_state(cell, CellState::SouthWall, true),
                    _ => {}
                }
            }
        }

        Ok(maze)
    }
}
