use anyhow::{anyhow, bail, Context, Result};
use bitflags::bitflags;
use pix_engine::{prelude::Color, rect, state::PixState};
use serde::{Deserialize, Serialize};

use crate::{
    engine::Render,
    simulator::{CELL_SIZE_VIS, WALL_LENGTH_VIS, WALL_WIDTH_VIS},
};

pub struct Posts<const R: usize, const C: usize>;

bitflags! {
    #[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct CellState: u8 {
        const NorthWall = 0b00000001;
        const EastWall = 0b00000010;
        const SouthWall = 0b00000100;
        const WestWall = 0b00001000;
        const Visited = 0b00010000;
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Cell<const R: usize, const C: usize> {
    pub x: usize,
    pub y: usize,
}

impl<const R: usize, const C: usize> Cell<R, C> {
    pub fn new(x: usize, y: usize) -> Result<Self> {
        if y >= R || x >= C {
            bail!("Coordinates out of bands");
        }

        Ok(Self { x, y })
    }
}

#[derive(Copy, Clone)]
pub struct Goal<const R: usize, const C: usize> {
    g0: Option<Cell<R, C>>,
    g1: Option<Cell<R, C>>,
    g2: Option<Cell<R, C>>,
    g3: Option<Cell<R, C>>,
}

impl<const R: usize, const C: usize> Goal<R, C> {
    pub fn new() -> Self {
        Self {
            g0: None,
            g1: None,
            g2: None,
            g3: None,
        }
    }

    pub fn set(&mut self, cell: Cell<R, C>) -> Result<()> {
        if self.g0.is_none() {
            self.g0 = Some(cell);

            Ok(())
        } else if self.g1.is_none() {
            self.g1 = Some(cell);

            Ok(())
        } else if self.g2.is_none() {
            self.g2 = Some(cell);

            Ok(())
        } else if self.g3.is_none() {
            self.g3 = Some(cell);

            Ok(())
        } else {
            Err(anyhow!("All 4 goal cells are set"))
        }
    }

    #[allow(dead_code)]
    pub fn is_target(&self, cell: Cell<R, C>) -> bool {
        let cell = Some(cell);

        if self.g0 == cell {
            true
        } else if self.g1 == cell {
            true
        } else if self.g2 == cell {
            true
        } else if self.g3 == cell {
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct Maze<const R: usize, const C: usize> {
    map: [[CellState; R]; C],
    goal: Goal<R, C>,
    start: Cell<R, C>,
}

impl<const R: usize, const C: usize> Maze<R, C> {
    pub fn new() -> Self {
        Self {
            map: [[CellState::default(); R]; C],
            goal: Goal::new(),
            start: Cell::new(0, 0).expect("Constructor should succeed"),
        }
    }

    pub fn get_cell_state(&self, cell: Cell<R, C>) -> CellState {
        self.map[cell.x][cell.y]
    }

    pub fn update_cell_state(&mut self, cell: Cell<R, C>, state: CellState, value: bool) {
        self.map[cell.x][cell.y].set(state, value);

        if state.contains(CellState::NorthWall) {
            if let Ok(neighbour) = Cell::<R, C>::new(cell.x, cell.y + 1) {
                self.map[neighbour.x][neighbour.y].set(CellState::SouthWall, value)
            }
        }

        if state.contains(CellState::SouthWall) && cell.y > 0 {
            if let Ok(neighbour) = Cell::<R, C>::new(cell.x, cell.y - 1) {
                self.map[neighbour.x][neighbour.y].set(CellState::NorthWall, value)
            }
        }

        if state.contains(CellState::EastWall) {
            if let Ok(neighbour) = Cell::<R, C>::new(cell.x + 1, cell.y) {
                self.map[neighbour.x][neighbour.y].set(CellState::WestWall, value)
            }
        }

        if state.contains(CellState::WestWall) && cell.x > 0 {
            if let Ok(neighbour) = Cell::<R, C>::new(cell.x - 1, cell.y) {
                self.map[neighbour.x][neighbour.y].set(CellState::EastWall, value)
            }
        }
    }

    pub fn set_goal_cell(&mut self, cell: Cell<R, C>) -> Result<()> {
        self.goal.set(cell)
    }

    pub fn set_start_cell(&mut self, cell: Cell<R, C>) {
        self.start = cell;
    }

    pub fn get_start_cell(&self) -> Cell<R, C> {
        self.start
    }
}

impl<const R: usize, const C: usize> Render for Maze<R, C> {
    fn draw<T>(&self, s: &mut PixState, primary_color: T, secondary_color: T) -> Result<()>
    where
        T: Into<Option<Color>> + std::marker::Copy,
    {
        s.stroke(secondary_color);
        s.fill(primary_color);

        for y in 0..R as i32 {
            for x in 0..C as i32 {
                let cell =
                    Cell::new(x as usize, y as usize).context("Coordinates should be in bounds")?;
                let cell_state = self.get_cell_state(cell);

                if cell_state.contains(CellState::Visited) {
                    s.stroke(None);
                    s.fill(Color::rgb(0x10, 0x10, 0x10));

                    s.rect(rect![
                        x * CELL_SIZE_VIS + WALL_WIDTH_VIS + 1,
                        (R as i32 - y - 1) * CELL_SIZE_VIS + WALL_WIDTH_VIS + 1,
                        WALL_LENGTH_VIS - 2,
                        WALL_LENGTH_VIS - 2,
                    ])?;

                    s.stroke(secondary_color);
                    s.fill(primary_color);
                }

                if cell_state.contains(CellState::NorthWall) {
                    s.rect(rect![
                        WALL_WIDTH_VIS + x * CELL_SIZE_VIS,
                        (R as i32 - y - 1) * CELL_SIZE_VIS,
                        WALL_LENGTH_VIS,
                        WALL_WIDTH_VIS,
                    ])?;
                }

                if cell_state.contains(CellState::WestWall) {
                    s.rect(rect![
                        x * CELL_SIZE_VIS,
                        WALL_WIDTH_VIS + (R as i32 - y - 1) * CELL_SIZE_VIS,
                        WALL_WIDTH_VIS,
                        WALL_LENGTH_VIS,
                    ])?;
                }

                if y == 0 {
                    if cell_state.contains(CellState::SouthWall) {
                        s.rect(rect![
                            WALL_WIDTH_VIS + x * CELL_SIZE_VIS,
                            (R as i32 - y - 1) * CELL_SIZE_VIS + CELL_SIZE_VIS,
                            WALL_LENGTH_VIS,
                            WALL_WIDTH_VIS,
                        ])?;
                    }
                }

                if x == C as i32 - 1 {
                    if cell_state.contains(CellState::EastWall) {
                        s.rect(rect![
                            x * CELL_SIZE_VIS + CELL_SIZE_VIS,
                            WALL_WIDTH_VIS + (R as i32 - y - 1) * CELL_SIZE_VIS,
                            WALL_WIDTH_VIS,
                            WALL_LENGTH_VIS,
                        ])?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<const R: usize, const C: usize> Render for Posts<R, C> {
    fn draw<T>(&self, s: &mut PixState, primary_color: T, secondary_color: T) -> Result<()>
    where
        T: Into<Option<Color>>,
    {
        s.stroke(secondary_color);
        s.fill(primary_color);

        for y in 0..R as i32 {
            for x in 0..C as i32 {
                s.rect(rect![
                    x * CELL_SIZE_VIS,
                    y * CELL_SIZE_VIS,
                    WALL_WIDTH_VIS,
                    WALL_WIDTH_VIS,
                ])?;

                if y == R as i32 - 1 {
                    s.rect(rect![
                        x * CELL_SIZE_VIS,
                        y * CELL_SIZE_VIS + CELL_SIZE_VIS,
                        WALL_WIDTH_VIS,
                        WALL_WIDTH_VIS,
                    ])?;
                }

                if x == C as i32 - 1 {
                    s.rect(rect![
                        x * CELL_SIZE_VIS + CELL_SIZE_VIS,
                        y * CELL_SIZE_VIS,
                        WALL_WIDTH_VIS,
                        WALL_WIDTH_VIS,
                    ])?;
                }

                if y == R as i32 - 1 && x == C as i32 - 1 {
                    s.rect(rect![
                        x * CELL_SIZE_VIS + CELL_SIZE_VIS,
                        y * CELL_SIZE_VIS + CELL_SIZE_VIS,
                        WALL_WIDTH_VIS,
                        WALL_WIDTH_VIS,
                    ])?;
                }
            }
        }

        Ok(())
    }
}
