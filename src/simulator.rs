use anyhow::{anyhow, Context, Result};
use pix_engine::prelude::Engine;
use std::{sync::mpsc, thread};

use crate::{communication::SimCommunication, engine::SimEngine, maze::Maze, COLS, ROWS};

pub const RATIO_VIS_MM: i32 = 4;

pub const CELL_SIZE_MM: i32 = 180;
pub const WALL_WIDTH_MM: i32 = 12;

pub const CELL_SIZE_VIS: i32 = CELL_SIZE_MM / RATIO_VIS_MM;
pub const WALL_WIDTH_VIS: i32 = WALL_WIDTH_MM / RATIO_VIS_MM;
pub const WALL_LENGTH_VIS: i32 = CELL_SIZE_VIS - WALL_WIDTH_VIS;

pub const APP_HEIGHT: u32 = CELL_SIZE_VIS as u32 * ROWS as u32 + WALL_WIDTH_VIS as u32;
pub const APP_WIDTH: u32 = CELL_SIZE_VIS as u32 * COLS as u32 + WALL_WIDTH_VIS as u32;

pub struct MazeSimulator<const R: usize, const C: usize>;

impl<const R: usize, const C: usize> MazeSimulator<R, C> {
    pub fn run(maze: Maze<R, C>) -> anyhow::Result<()> {
        let (request_tx, request_rx) = mpsc::channel();
        let (response_tx, response_rx) = mpsc::channel();

        let _ = thread::spawn(move || {
            let process = || -> Result<()> {
                loop {
                    match request_rx.try_recv() {
                        Ok(request) => {
                            println!("{:?}", request);
                            response_tx
                                .send(crate::communication::MazeRunnerResponse::Error)
                                .context("Failed to propagate response")?
                        }
                        Err(mpsc::TryRecvError::Empty) => {}
                        Err(e) => return Err(anyhow!("Channel dropped: {e}")),
                    }
                }
            };

            process().unwrap();
        });

        let communication = SimCommunication::new(request_tx, response_rx)?;

        let _ = thread::spawn(move || communication.process().unwrap());

        let mut engine = SimEngine::new(maze);

        let mut pix_engine = Engine::builder()
            .dimensions(APP_WIDTH + 1, APP_HEIGHT + 1)
            .title("Maze Simulator")
            .target_frame_rate(60)
            .build()?;

        pix_engine.run(&mut engine)
    }
}
