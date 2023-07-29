use anyhow::Result;
use pix_engine::prelude::*;

use crate::maze::{Maze, Posts};

pub trait Render {
    fn draw(&self, s: &mut PixState) -> Result<()>;
}

pub struct SimEngine<const R: usize, const C: usize> {
    posts: Posts<R, C>,
    maze: Maze<R, C>,
}

impl<const R: usize, const C: usize> SimEngine<R, C> {
    pub fn new(maze: Maze<R, C>) -> Self {
        Self {
            maze,
            posts: Posts {},
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

        s.stroke(Color::DARK_GRAY);
        s.fill(Color::DIM_GRAY);

        self.posts.draw(s)?;
        self.maze.draw(s)?;

        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}
