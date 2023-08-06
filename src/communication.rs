use anyhow::{Context, Result};
use bitflags::bitflags;
use postcard::{from_bytes, to_stdvec};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::{Receiver, Sender};

use crate::maze::CellState;

const SOCKET: &str = "/tmp/micromouse_simulator_socket";

bitflags! {
    #[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct ButtonsState: u8 {
        const Reset = 0b00000001;
        const Button1 = 0b00000010;
        const Button2 = 0b00000100;
        const Button3 = 0b00001000;
        const Button4 = 0b00010000;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MazeRunnerRequest {
    Initialize,
    MoveForward,
    RotateRight90,
    RotateLeft90,
    GetWallFront,
    GetWallRight,
    GetWallLeft,
    GetButtonsState,
    UpdateCellState {
        x: usize,
        y: usize,
        state: CellState,
    },
    ClearCell {
        x: usize,
        y: usize,
    },
    UpdateCellValue {
        x: usize,
        y: usize,
        value: i32,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MazeRunnerResponse {
    Ack,
    Error,
    WallDetected(bool),
    Buttons(ButtonsState),
}
pub struct SimCommunication {
    listener: UnixListener,
    request_tx: Sender<MazeRunnerRequest>,
    response_rx: Receiver<MazeRunnerResponse>,
}

impl SimCommunication {
    pub fn new(
        request_tx: Sender<MazeRunnerRequest>,
        response_rx: Receiver<MazeRunnerResponse>,
    ) -> Result<Self> {
        if std::fs::metadata(SOCKET).is_ok() {
            std::fs::remove_file(SOCKET).context("Failed to remove existing socket")?;
        }

        let listener = UnixListener::bind(SOCKET).context("Failed to create socket")?;

        Ok(Self {
            listener,
            request_tx,
            response_rx,
        })
    }

    pub fn process(mut self) -> Result<()> {
        loop {
            let (stream, _) = self
                .listener
                .accept()
                .context("Failed to accept connection")?;

            self.handle_stream(stream)?;
        }
    }

    pub fn handle_stream(&mut self, mut stream: UnixStream) -> Result<()> {
        loop {
            let mut buffer = [0; 100];

            let n = stream
                .read(&mut buffer[..])
                .context("Failed to receive request")?;

            if n == 0 {
                return Ok(());
            }

            let request: MazeRunnerRequest =
                from_bytes(&buffer).context("Failed to deserialize request")?;

            self.request_tx
                .send(request)
                .context("Failed to propagate request")?;

            let response = self.response_rx.recv().context("Failed to get response")?;

            let response_buffer: Vec<u8> =
                to_stdvec(&response).context("Failed to serialize response")?;

            stream
                .write_all(response_buffer.as_slice())
                .context("Failed to send response")?;

            stream.flush().context("Could not flush the stream: {e}")?;
        }
    }
}
