use anyhow::{anyhow, Context, Result};
use std::sync::{
    mpsc::{Receiver, Sender, TryRecvError},
    Arc, Mutex,
};

use crate::{
    communication::{MazeRunnerRequest, MazeRunnerResponse},
    position::{Angle, Position},
};

pub struct SimEnvironment<const R: usize, const C: usize> {
    request_rx: Receiver<MazeRunnerRequest>,
    response_tx: Sender<MazeRunnerResponse>,
    runner_position: Arc<Mutex<Position<R>>>,
}

impl<const R: usize, const C: usize> SimEnvironment<R, C> {
    pub fn new(
        request_rx: Receiver<MazeRunnerRequest>,
        response_tx: Sender<MazeRunnerResponse>,
    ) -> Result<Self> {
        let runner_position = Position::new(270.0, 270.0, Angle::degrees(0.0));

        let runner_position = Arc::new(Mutex::new(runner_position));

        Ok(Self {
            request_rx,
            response_tx,
            runner_position,
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

        let response = MazeRunnerResponse::Error;

        self.response_tx
            .send(response)
            .context("Failed to propagate response")
    }
}
