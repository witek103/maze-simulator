use anyhow::{anyhow, Context, Result};
use std::sync::{
    mpsc::{Receiver, Sender, TryRecvError},
    Arc, Mutex,
};

use crate::{
    communication::{MazeRunnerRequest, MazeRunnerResponse},
    maze::Maze,
    position::{Angle, Position},
    runner::{MazerRunner, SensorDirection},
};

pub struct SimEnvironment<const R: usize, const C: usize> {
    maze: Maze<R, C>,
    request_rx: Receiver<MazeRunnerRequest>,
    response_tx: Sender<MazeRunnerResponse>,
    runner_position: Arc<Mutex<Position<R>>>,
    runner: MazerRunner<R, C>,
}

impl<const R: usize, const C: usize> SimEnvironment<R, C> {
    pub fn new(
        maze: Maze<R, C>,
        request_rx: Receiver<MazeRunnerRequest>,
        response_tx: Sender<MazeRunnerResponse>,
    ) -> Result<Self> {
        let runner = MazerRunner::new(&maze)?;

        let runner_position = Position::new(270.0, 270.0, Angle::degrees(0.0));

        let runner_position = Arc::new(Mutex::new(runner_position));

        Ok(Self {
            maze,
            request_rx,
            response_tx,
            runner_position,
            runner,
        })
    }

    pub fn process(mut self) -> Result<()> {
        loop {
            match self.request_rx.try_recv() {
                Ok(request) => self.process_request(request)?,
                Err(TryRecvError::Empty) => {}
                Err(e) => return Err(anyhow!("Channel dropped: {e}")),
            };
        }
    }

    pub fn get_runner_position_handle(&self) -> Arc<Mutex<Position<R>>> {
        self.runner_position.clone()
    }

    fn process_request(&mut self, request: MazeRunnerRequest) -> Result<()> {
        println!("{:?}", request);

        let response = match request {
            MazeRunnerRequest::Initialize => self.process_initialize()?,
            MazeRunnerRequest::GetWallFront => MazeRunnerResponse::WallDetected(
                self.runner
                    .is_wall_detected(&self.maze, SensorDirection::Front),
            ),
            MazeRunnerRequest::GetWallLeft => MazeRunnerResponse::WallDetected(
                self.runner
                    .is_wall_detected(&self.maze, SensorDirection::Left),
            ),
            MazeRunnerRequest::GetWallRight => MazeRunnerResponse::WallDetected(
                self.runner
                    .is_wall_detected(&self.maze, SensorDirection::Right),
            ),
            _ => MazeRunnerResponse::Error,
        };

        self.response_tx
            .send(response)
            .context("Failed to propagate response")
    }

    fn process_initialize(&mut self) -> Result<MazeRunnerResponse> {
        self.runner = MazerRunner::new(&self.maze)?;

        let mut runner_position = self.runner_position.lock().unwrap();

        *runner_position = self.runner.get_real_position();

        Ok(MazeRunnerResponse::Ack)
    }
}
