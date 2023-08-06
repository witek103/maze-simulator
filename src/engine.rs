use anyhow::Result;
use pix_engine::prelude::*;

use crate::maze::{Maze, Posts};
use crate::position::{Angle, Position};

pub trait Render {
    fn draw<C>(&self, s: &mut PixState, primary_color: C, secondary_color: C) -> Result<()>
    where
        C: Into<Option<Color>>;
}

pub struct SimEngine<const R: usize, const C: usize> {
    posts: Posts<R, C>,
    maze: Maze<R, C>,
    runner_position: Position<R>,
}

impl<const R: usize, const C: usize> SimEngine<R, C> {
    pub fn new(maze: Maze<R, C>) -> Self {
        let runner_position = Position::new(270.0, 270.0, Angle::degrees(0.0));

        Self {
            maze,
            posts: Posts {},
            runner_position,
        }
    }
}

impl<const R: usize, const C: usize> PixEngine for SimEngine<R, C> {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(Color::BLACK);

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;

        self.posts.draw(s, Color::DIM_GRAY, Color::DARK_GRAY)?;
        self.maze.draw(s, Color::DIM_GRAY, Color::DARK_GRAY)?;

        self.runner_position
            .draw(s, Color::DARK_GREEN, Color::LIGHT_GREEN)?;

        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}
