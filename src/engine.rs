use anyhow::Result;
use pix_engine::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    communication::ButtonsState,
    maze::{Maze, Posts},
    panel::SimPanel,
    position::Position,
};

pub trait Render {
    fn draw<C>(&self, s: &mut PixState, primary_color: C, secondary_color: C) -> Result<()>
    where
        C: Into<Option<Color>> + std::marker::Copy;
}

pub struct SimEngine<const R: usize, const C: usize, T> {
    posts: Posts<R, C>,
    maze: Maze<R, C>,
    runner_position: Arc<Mutex<Position<R>>>,
    panel: SimPanel,
    runner_context: Arc<Mutex<T>>,
}

impl<const R: usize, const C: usize, T: Render> SimEngine<R, C, T> {
    pub fn new(
        maze: Maze<R, C>,
        runner_position: Arc<Mutex<Position<R>>>,
        buttons: Arc<Mutex<ButtonsState>>,
        runner_context: Arc<Mutex<T>>,
    ) -> Self {
        Self {
            maze,
            posts: Posts {},
            runner_position,
            panel: SimPanel::new(buttons),
            runner_context,
        }
    }
}

impl<const R: usize, const C: usize, T: Render> PixEngine for SimEngine<R, C, T> {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(Color::BLACK);

        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;

        self.posts.draw(s, Color::DIM_GRAY, Color::DARK_GRAY)?;
        self.maze.draw(s, Color::DIM_GRAY, Color::DARK_GRAY)?;

        self.runner_context
            .lock()
            .unwrap()
            .draw(s, Color::RED, Color::DARK_GRAY)?;

        {
            self.runner_position
                .lock()
                .unwrap()
                .draw(s, Color::DARK_GREEN, Color::LIGHT_GREEN)?;
        }

        self.panel.draw(s, Color::DIM_GRAY, Color::DARK_GRAY)?;

        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}
