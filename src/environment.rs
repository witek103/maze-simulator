use anyhow::{anyhow, Context, Result};
use std::{
    sync::{
        mpsc::{Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread::sleep,
    time::Duration,
};

use crate::{
    communication::{ButtonsState, MazeRunnerRequest, MazeRunnerResponse},
    maze::Maze,
    position::{Angle, Position},
    runner::{MazerRunner, RotationDirection, SensorDirection},
};

const MOVEMENT_TIME: usize = 400;
const ROTATION_TIME: usize = 200;

pub struct SimEnvironment<const R: usize, const C: usize> {
    maze: Maze<R, C>,
    request_rx: Receiver<MazeRunnerRequest>,
    response_tx: Sender<MazeRunnerResponse>,
    runner_position: Arc<Mutex<Position<R>>>,
    runner: MazerRunner<R, C>,
    buttons: Arc<Mutex<ButtonsState>>,
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

        let buttons = Arc::new(Mutex::new(ButtonsState::default()));

        Ok(Self {
            maze,
            request_rx,
            response_tx,
            runner_position,
            runner,
            buttons,
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

    pub fn get_buttons_handle(&self) -> Arc<Mutex<ButtonsState>> {
        self.buttons.clone()
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
            MazeRunnerRequest::MoveForward => self.process_move_forward(),
            MazeRunnerRequest::RotateLeft90 => self.process_rotate(RotationDirection::Left),
            MazeRunnerRequest::RotateRight90 => self.process_rotate(RotationDirection::Right),
            MazeRunnerRequest::GetButtonsState => self.process_buttons(),
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

    fn process_move_forward(&mut self) -> MazeRunnerResponse {
        if self.runner.move_forward(&self.maze).is_err() {
            return MazeRunnerResponse::Error;
        }

        let next_position = self.runner.get_real_position();

        let position = self.runner_position.lock().unwrap().clone();

        let x_step = (next_position.x - position.x) / MOVEMENT_TIME as f64;
        let y_step = (next_position.y - position.y) / MOVEMENT_TIME as f64;

        for _ in 1..MOVEMENT_TIME + 1 {
            sleep(Duration::from_millis(1));

            let mut runner_position = self.runner_position.lock().unwrap();

            runner_position.x += x_step;
            runner_position.y += y_step;
        }

        MazeRunnerResponse::Ack
    }

    fn process_rotate(&mut self, direction: RotationDirection) -> MazeRunnerResponse {
        self.runner.rotate(direction);

        let r_step = match direction {
            RotationDirection::Left => Angle::degrees(90.0),
            RotationDirection::Right => Angle::degrees(-90.0),
        } / ROTATION_TIME as f64;

        for _ in 1..ROTATION_TIME + 1 {
            sleep(Duration::from_millis(1));

            let mut runner_position = self.runner_position.lock().unwrap();

            runner_position.theta = runner_position.theta + r_step;
        }

        MazeRunnerResponse::Ack
    }

    fn process_buttons(&self) -> MazeRunnerResponse {
        let mut buttons = self.buttons.lock().unwrap();

        let response = buttons.clone();

        buttons.remove(ButtonsState::all());

        MazeRunnerResponse::Buttons(response)
    }
}
