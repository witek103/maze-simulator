use anyhow::Result;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::position::{Angle, Position};

#[derive(Clone)]
pub struct Velocity {
    pub translational: f64,
    pub rotational: f64,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            translational: 0.0,
            rotational: 0.0,
        }
    }
}

pub struct VelocityEnvironment<const R: usize> {
    runner_position: Arc<Mutex<Position<R>>>,
    velocity: Arc<Mutex<Velocity>>,
}

impl<const R: usize> VelocityEnvironment<R> {
    pub fn new(runner_position: Arc<Mutex<Position<R>>>, velocity: Arc<Mutex<Velocity>>) -> Self {
        Self {
            runner_position,
            velocity,
        }
    }

    pub fn process(self) -> Result<()> {
        loop {
            {
                let mut runner_position = self.runner_position.lock().unwrap();

                let velocity = self.velocity.lock().unwrap().clone();

                let delta_x = velocity.translational * runner_position.theta.cos() * 0.0001;
                let delta_y = velocity.translational * runner_position.theta.sin() * 0.0001;
                let delta_theta = velocity.rotational * 0.0001;

                runner_position.x += delta_x;
                runner_position.y += delta_y;
                runner_position.theta = runner_position.theta + Angle::radians(delta_theta);
            }

            sleep(Duration::from_micros(100));
        }
    }
}
