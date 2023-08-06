use anyhow::{anyhow, Context, Result};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use crate::communication::{MazeRunnerRequest, MazeRunnerResponse};

pub struct SimEnvironment<const R: usize, const C: usize> {
    request_rx: Receiver<MazeRunnerRequest>,
    response_tx: Sender<MazeRunnerResponse>,
}

impl<const R: usize, const C: usize> SimEnvironment<R, C> {
    pub fn new(
        request_rx: Receiver<MazeRunnerRequest>,
        response_tx: Sender<MazeRunnerResponse>,
    ) -> Result<Self> {
        Ok(Self {
            request_rx,
            response_tx,
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

    fn process_request(&mut self, request: MazeRunnerRequest) -> Result<()> {
        println!("{:?}", request);

        let response = MazeRunnerResponse::Error;

        self.response_tx
            .send(response)
            .context("Failed to propagate response")
    }
}
