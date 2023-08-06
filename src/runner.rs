use anyhow::{bail, Result};

use crate::{
    maze::{Cell, CellState, Maze},
    position::{Angle, Position},
    simulator::CELL_SIZE_MM,
};

pub enum MazeOrientation {
    North,
    East,
    South,
    West,
}

pub struct MazerRunner<const R: usize, const C: usize> {
    cell: Cell<R, C>,
    orientation: MazeOrientation,
}

impl<const R: usize, const C: usize> MazerRunner<R, C> {
    pub fn new(maze: &Maze<R, C>) -> Result<Self> {
        let start_cell = maze.get_start_cell();
        let start_cell_state = maze.get_cell_state(start_cell);

        let orientation = if !start_cell_state.contains(CellState::NorthWall) {
            MazeOrientation::North
        } else if !start_cell_state.contains(CellState::EastWall) {
            MazeOrientation::East
        } else if !start_cell_state.contains(CellState::SouthWall) {
            MazeOrientation::South
        } else if !start_cell_state.contains(CellState::WestWall) {
            MazeOrientation::West
        } else {
            bail!("Starting cell is blocked");
        };

        Ok(Self {
            cell: start_cell,
            orientation,
        })
    }

    pub fn get_real_position(&self) -> Position<R> {
        let x = self.cell.x as f64 * CELL_SIZE_MM as f64 + CELL_SIZE_MM as f64 / 2.0;
        let y = self.cell.y as f64 * CELL_SIZE_MM as f64 + CELL_SIZE_MM as f64 / 2.0;

        let theta = match self.orientation {
            MazeOrientation::North => Angle::degrees(90.0),
            MazeOrientation::East => Angle::degrees(0.0),
            MazeOrientation::South => Angle::degrees(-90.0),
            MazeOrientation::West => Angle::degrees(180.0),
        };

        Position::new(x, y, theta)
    }
}
