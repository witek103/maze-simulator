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

pub enum SensorDirection {
    Front,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub enum RotationDirection {
    Left,
    Right,
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

    pub fn is_wall_detected(&self, maze: &Maze<R, C>, direction: SensorDirection) -> bool {
        let cell_state = maze.get_cell_state(self.cell);

        match direction {
            SensorDirection::Front => match self.orientation {
                MazeOrientation::East => cell_state.contains(CellState::EastWall),
                MazeOrientation::North => cell_state.contains(CellState::NorthWall),
                MazeOrientation::West => cell_state.contains(CellState::WestWall),
                MazeOrientation::South => cell_state.contains(CellState::SouthWall),
            },

            SensorDirection::Left => match self.orientation {
                MazeOrientation::East => cell_state.contains(CellState::NorthWall),
                MazeOrientation::North => cell_state.contains(CellState::WestWall),
                MazeOrientation::West => cell_state.contains(CellState::SouthWall),
                MazeOrientation::South => cell_state.contains(CellState::EastWall),
            },
            SensorDirection::Right => match self.orientation {
                MazeOrientation::East => cell_state.contains(CellState::SouthWall),
                MazeOrientation::North => cell_state.contains(CellState::EastWall),
                MazeOrientation::West => cell_state.contains(CellState::NorthWall),
                MazeOrientation::South => cell_state.contains(CellState::WestWall),
            },
        }
    }

    pub fn move_forward(&mut self, maze: &Maze<R, C>) -> Result<()> {
        if self.is_wall_detected(maze, SensorDirection::Front) {
            bail!("Wall in front of Runner");
        }

        match self.orientation {
            MazeOrientation::North => self.cell.y += 1,
            MazeOrientation::East => self.cell.x += 1,
            MazeOrientation::South => self.cell.y -= 1,
            MazeOrientation::West => self.cell.x -= 1,
        }

        Ok(())
    }

    pub fn rotate(&mut self, direction: RotationDirection) {
        self.orientation = match direction {
            RotationDirection::Left => match self.orientation {
                MazeOrientation::East => MazeOrientation::North,
                MazeOrientation::North => MazeOrientation::West,
                MazeOrientation::West => MazeOrientation::South,
                MazeOrientation::South => MazeOrientation::East,
            },
            RotationDirection::Right => match self.orientation {
                MazeOrientation::East => MazeOrientation::South,
                MazeOrientation::North => MazeOrientation::East,
                MazeOrientation::West => MazeOrientation::North,
                MazeOrientation::South => MazeOrientation::West,
            },
        };
    }
}
