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
    communication::{
        ButtonsState, DistanceSensor, MazeRunnerRequest, MazeRunnerResponse, MotionReadout,
    },
    context::RunnerContext,
    distance_sensors::DistanceSensorsReading,
    maze::{Cell, CellState, Maze},
    position::{Angle, Position},
    runner::{MazerRunner, RotationDirection, SensorDirection},
    velocity::Velocity,
};

const TRANSLATIONAL_VELOCITY: f64 = 400.0; // 400.0 [mm/s]
const ROTATIONAL_VELOCITY: f64 = 6.98131701; // ~400 [deg/s]

pub struct SimEnvironment<const R: usize, const C: usize> {
    maze: Maze<R, C>,
    request_rx: Receiver<MazeRunnerRequest>,
    response_tx: Sender<MazeRunnerResponse>,
    runner_position: Arc<Mutex<Position<R>>>,
    runner: MazerRunner<R, C>,
    buttons: Arc<Mutex<ButtonsState>>,
    runner_context: Arc<Mutex<RunnerContext<R, C>>>,
    distance_sensors: Arc<Mutex<DistanceSensorsReading>>,
    velocity: Arc<Mutex<Velocity>>,
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

        let distance_sensors = Arc::new(Mutex::new(DistanceSensorsReading::new()));

        let buttons = Arc::new(Mutex::new(ButtonsState::default()));

        let runner_context = Arc::new(Mutex::new(RunnerContext::new()));

        let velocity = Arc::new(Mutex::new(Velocity::new()));

        Ok(Self {
            maze,
            request_rx,
            response_tx,
            runner_position,
            runner,
            buttons,
            runner_context,
            distance_sensors,
            velocity,
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

    pub fn get_runner_context_handle(&self) -> Arc<Mutex<RunnerContext<R, C>>> {
        self.runner_context.clone()
    }

    pub fn get_distance_sensors_handle(&self) -> Arc<Mutex<DistanceSensorsReading>> {
        self.distance_sensors.clone()
    }

    pub fn get_velocity_handle(&self) -> Arc<Mutex<Velocity>> {
        self.velocity.clone()
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
            MazeRunnerRequest::UpdateCellState { x, y, state } => {
                self.process_update_cell_state(x, y, state)
            }

            MazeRunnerRequest::ClearCell { x, y } => self.process_clear_cell(x, y),
            MazeRunnerRequest::UpdateCellValue { x, y, value } => {
                self.process_update_cell_value(x, y, value)
            }
            MazeRunnerRequest::GetDistanceReadout { sensor } => {
                self.process_distance_readout(sensor)
            }
            MazeRunnerRequest::GetMotionReadout => self.process_motion_readout(),
            MazeRunnerRequest::SetVelocity {
                translational,
                rotational,
            } => self.process_set_velocity(translational, rotational),
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

        {
            let mut velocity = self.velocity.lock().unwrap();

            velocity.translational = TRANSLATIONAL_VELOCITY;
            velocity.rotational = 0.0;
        }

        loop {
            sleep(Duration::from_micros(100));

            let mut runner_position = self.runner_position.lock().unwrap();

            if (runner_position.x - next_position.x).abs() < 2.0
                && (runner_position.y - next_position.y).abs() < 2.0
            {
                runner_position.x = next_position.x;
                runner_position.y = next_position.y;

                let mut velocity = self.velocity.lock().unwrap();

                velocity.translational = 0.0;

                break;
            }
        }

        MazeRunnerResponse::Ack
    }

    fn process_rotate(&mut self, direction: RotationDirection) -> MazeRunnerResponse {
        self.runner.rotate(direction);

        let next_position = self.runner.get_real_position();

        {
            let mut velocity = self.velocity.lock().unwrap();

            velocity.translational = 0.0;

            velocity.rotational = match direction {
                RotationDirection::Left => ROTATIONAL_VELOCITY,
                RotationDirection::Right => -ROTATIONAL_VELOCITY,
            };
        }

        loop {
            sleep(Duration::from_micros(100));

            let mut runner_position = self.runner_position.lock().unwrap();

            if runner_position
                .theta
                .is_within(&next_position.theta, Angle::degrees(1.0))
            {
                runner_position.theta = next_position.theta;

                let mut velocity = self.velocity.lock().unwrap();

                velocity.rotational = 0.0;

                break;
            }
        }

        MazeRunnerResponse::Ack
    }

    fn process_buttons(&self) -> MazeRunnerResponse {
        let mut buttons = self.buttons.lock().unwrap();

        let response = buttons.clone();

        buttons.remove(ButtonsState::all());

        MazeRunnerResponse::Buttons(response)
    }

    fn process_clear_cell(&mut self, x: usize, y: usize) -> MazeRunnerResponse {
        let cell = match Cell::new(x, y) {
            Ok(cell) => cell,
            Err(_) => return MazeRunnerResponse::Error,
        };

        self.runner_context.lock().unwrap().clear_cell(cell);

        MazeRunnerResponse::Ack
    }

    fn process_update_cell_state(
        &mut self,
        x: usize,
        y: usize,
        state: CellState,
    ) -> MazeRunnerResponse {
        let cell = match Cell::new(x, y) {
            Ok(cell) => cell,
            Err(_) => return MazeRunnerResponse::Error,
        };

        self.runner_context
            .lock()
            .unwrap()
            .set_cell_state(cell, state);

        MazeRunnerResponse::Ack
    }

    fn process_update_cell_value(&mut self, x: usize, y: usize, value: i32) -> MazeRunnerResponse {
        let cell = match Cell::new(x, y) {
            Ok(cell) => cell,
            Err(_) => return MazeRunnerResponse::Error,
        };

        self.runner_context
            .lock()
            .unwrap()
            .set_cell_value(cell, value);

        MazeRunnerResponse::Ack
    }

    fn process_distance_readout(&self, sensor: DistanceSensor) -> MazeRunnerResponse {
        let distance_sensors = self.distance_sensors.lock().unwrap().clone();

        let distance = match sensor {
            DistanceSensor::FrontLeft => distance_sensors.fl,
            DistanceSensor::FrontRight => distance_sensors.fr,
            DistanceSensor::DiagonalLeft => distance_sensors.dl,
            DistanceSensor::DiagonalRight => distance_sensors.dr,
        };

        MazeRunnerResponse::Distance(distance as u16)
    }

    fn process_motion_readout(&self) -> MazeRunnerResponse {
        let position = self.runner_position.lock().unwrap().clone();
        let velocity = self.velocity.lock().unwrap().clone();

        MazeRunnerResponse::Motion(MotionReadout {
            x: position.x as i32,
            y: position.y as i32,
            theta: position.theta.as_degrees(),
            velocity_translational: velocity.translational,
            velocity_rotational: velocity.rotational,
        })
    }

    fn process_set_velocity(&self, translational: f64, rotational: f64) -> MazeRunnerResponse {
        let mut velocity = self.velocity.lock().unwrap();

        velocity.translational = translational;
        velocity.rotational = rotational;

        MazeRunnerResponse::Ack
    }
}
