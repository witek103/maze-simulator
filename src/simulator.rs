use pix_engine::prelude::Engine;
use std::{sync::mpsc, thread};

use crate::{
    communication::SimCommunication,
    distance_sensors::{
        DistanceSensorDiagonalLeft, DistanceSensorDiagonalRight, DistanceSensorFrontLeft,
        DistanceSensorFrontRight, DistanceSensorsEnvironment,
    },
    engine::SimEngine,
    environment::SimEnvironment,
    maze::Maze,
    COLS, ROWS,
};

pub const RATIO_VIS_MM: i32 = 4;

pub const CELL_SIZE_MM: i32 = 180;
pub const WALL_WIDTH_MM: i32 = 12;

pub const CELL_SIZE_VIS: i32 = CELL_SIZE_MM / RATIO_VIS_MM;
pub const WALL_WIDTH_VIS: i32 = WALL_WIDTH_MM / RATIO_VIS_MM;
pub const WALL_LENGTH_VIS: i32 = CELL_SIZE_VIS - WALL_WIDTH_VIS;

pub const PANEL_WIDTH: i32 = 400;

pub const APP_HEIGHT: u32 = CELL_SIZE_VIS as u32 * ROWS as u32 + WALL_WIDTH_VIS as u32;
pub const APP_WIDTH: u32 =
    CELL_SIZE_VIS as u32 * COLS as u32 + WALL_WIDTH_VIS as u32 + PANEL_WIDTH as u32;

pub struct MazeSimulator<const R: usize, const C: usize>;

impl<const R: usize, const C: usize> MazeSimulator<R, C> {
    pub fn run(maze: Maze<R, C>) -> anyhow::Result<()> {
        let (request_tx, request_rx) = mpsc::channel();
        let (response_tx, response_rx) = mpsc::channel();

        let environment = SimEnvironment::new(maze.clone(), request_rx, response_tx)?;

        let runner_position = environment.get_runner_position_handle();
        let buttons = environment.get_buttons_handle();
        let runner_context = environment.get_runner_context_handle();
        let distance_sensors = environment.get_distance_sensors_handle();

        let _ = thread::spawn(move || environment.process().unwrap());

        let communication = SimCommunication::new(request_tx, response_rx)?;

        let _ = thread::spawn(move || communication.process().unwrap());

        let distance_senors_environment = DistanceSensorsEnvironment::<
            R,
            C,
            DistanceSensorFrontLeft,
            DistanceSensorFrontRight,
            DistanceSensorDiagonalLeft,
            DistanceSensorDiagonalRight,
        >::new(
            maze.clone(),
            runner_position.clone(),
            distance_sensors.clone(),
        );

        let _ = thread::spawn(move || distance_senors_environment.process().unwrap());

        let mut engine = SimEngine::<R, C, _, _, _>::new(
            maze,
            runner_position,
            buttons,
            runner_context,
            distance_sensors,
        );

        let mut pix_engine = Engine::builder()
            .dimensions(APP_WIDTH + 1, APP_HEIGHT + 1)
            .title("Maze Simulator")
            .target_frame_rate(60)
            .build()?;

        pix_engine.run(&mut engine)
    }
}
