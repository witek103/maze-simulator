use anyhow::Result;
use pix_engine::prelude::*;
use std::sync::{Arc, Mutex};

use crate::{
    communication::ButtonsState, distance_sensors::DistanceSensorsReading, maze::Posts,
    panel::SimPanel,
};

pub trait Render {
    fn draw<C>(&self, s: &mut PixState, primary_color: C, secondary_color: C) -> Result<()>
    where
        C: Into<Option<Color>> + std::marker::Copy;
}

pub struct SimEngine<const R: usize, const C: usize, T, S, U> {
    posts: Posts<R, C>,
    maze: S,
    runner_position: Arc<Mutex<U>>,
    panel: SimPanel,
    runner_context: Arc<Mutex<T>>,
    distance_sensors: Arc<Mutex<DistanceSensorsReading>>,
}

impl<const R: usize, const C: usize, T, S, U> SimEngine<R, C, T, S, U>
where
    T: Render,
    S: Render,
    U: Render,
{
    pub fn new(
        maze: S,
        runner_position: Arc<Mutex<U>>,
        buttons: Arc<Mutex<ButtonsState>>,
        runner_context: Arc<Mutex<T>>,
        distance_sensors: Arc<Mutex<DistanceSensorsReading>>,
    ) -> Self {
        Self {
            maze,
            posts: Posts {},
            runner_position,
            panel: SimPanel::new(buttons, distance_sensors.clone()),
            runner_context,
            distance_sensors,
        }
    }
}

impl<const R: usize, const C: usize, T, S, U> PixEngine for SimEngine<R, C, T, S, U>
where
    T: Render,
    S: Render,
    U: Render,
{
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

        self.runner_position
            .lock()
            .unwrap()
            .draw(s, Color::DARK_GREEN, Color::LIGHT_GREEN)?;

        self.distance_sensors
            .lock()
            .unwrap()
            .draw(s, Color::ORANGE_RED, Color::ORANGE_RED)?;

        self.panel.draw(s, Color::DIM_GRAY, Color::DARK_GRAY)?;

        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}
