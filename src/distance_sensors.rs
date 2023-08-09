use anyhow::Result;
use pix_engine::{line_, shape::Line};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::{
    engine::Render,
    maze::{Cell, CellState, Maze},
    position::{Angle, Millimeters, Position},
    simulator::{CELL_SIZE_MM, RATIO_VIS_MM, WALL_WIDTH_MM},
};

pub trait DistanceSensor {
    fn alpha() -> Angle;
    fn position_x_offset() -> Millimeters;
    fn position_y_offset() -> Millimeters;
}

pub struct DistanceSensorsEnvironment<const R: usize, const C: usize, FL, FR, DL, DR> {
    maze: Maze<R, C>,
    runner_position: Arc<Mutex<Position<R>>>,
    distance_sensors: Arc<Mutex<DistanceSensorsReading>>,
    phantom_fl: PhantomData<FL>,
    phantom_fr: PhantomData<FR>,
    phantom_dl: PhantomData<DL>,
    phantom_dr: PhantomData<DR>,
}

impl<const R: usize, const C: usize, FL, FR, DL, DR>
    DistanceSensorsEnvironment<R, C, FL, FR, DL, DR>
where
    FL: DistanceSensor,
    FR: DistanceSensor,
    DL: DistanceSensor,
    DR: DistanceSensor,
{
    pub fn new(
        maze: Maze<R, C>,
        runner_position: Arc<Mutex<Position<R>>>,
        distance_sensors: Arc<Mutex<DistanceSensorsReading>>,
    ) -> Self {
        Self {
            maze,
            runner_position,
            distance_sensors,
            phantom_fl: PhantomData,
            phantom_fr: PhantomData,
            phantom_dl: PhantomData,
            phantom_dr: PhantomData,
        }
    }

    pub fn process(self) -> Result<()> {
        loop {
            {
                let runner_position = self.runner_position.lock().unwrap().clone();
                let mut distance_sensors = self.distance_sensors.lock().unwrap();

                (distance_sensors.fl, distance_sensors.fl_beam) =
                    self.front_left(&runner_position)?;

                (distance_sensors.fr, distance_sensors.fr_beam) =
                    self.front_right(&runner_position)?;

                (distance_sensors.dl, distance_sensors.dl_beam) =
                    self.diagonal_left(&runner_position)?;

                (distance_sensors.dr, distance_sensors.dr_beam) =
                    self.diagonal_right(&runner_position)?;
            }

            sleep(Duration::from_millis(20));
        }
    }

    pub fn is_wall_at(
        &self,
        x_index: usize,
        y_index: usize,
        x_offset: i32,
        y_offset: i32,
    ) -> Result<bool> {
        let cell_state = self.maze.get_cell_state(Cell::new(x_index, y_index)?);

        if cell_state.contains(CellState::NorthWall)
            && y_offset >= CELL_SIZE_MM - (WALL_WIDTH_MM / 2)
        {
            return Ok(true);
        }

        if cell_state.contains(CellState::SouthWall) && y_offset <= (WALL_WIDTH_MM / 2) {
            return Ok(true);
        }

        if cell_state.contains(CellState::EastWall)
            && x_offset >= CELL_SIZE_MM - (WALL_WIDTH_MM / 2)
        {
            return Ok(true);
        }

        if cell_state.contains(CellState::WestWall) && x_offset <= (WALL_WIDTH_MM / 2) {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn estimate_measured_distance<DS>(
        &self,
        runner_position: &Position<R>,
    ) -> Result<(i32, Line)>
    where
        DS: DistanceSensor,
    {
        let cos = (runner_position.theta + DS::alpha()).cos();
        let sin = (runner_position.theta + DS::alpha()).sin();

        let sensor_x = runner_position.x
            + DS::position_y_offset() * runner_position.theta.cos()
            + DS::position_x_offset() * (runner_position.theta + Angle::degrees(90.0)).cos();
        let sensor_y = runner_position.y
            + DS::position_y_offset() * runner_position.theta.sin()
            + DS::position_x_offset() * (runner_position.theta + Angle::degrees(90.0)).sin();

        for distance in 1..101 {
            let detection_x = sensor_x + distance as f64 * cos;
            let detection_y = sensor_y + distance as f64 * sin;

            let detection_x_index = detection_x as i32 / CELL_SIZE_MM;
            let detection_y_index = detection_y as i32 / CELL_SIZE_MM;

            let detection_x_offset = detection_x as i32 % CELL_SIZE_MM;
            let detection_y_offset = detection_y as i32 % CELL_SIZE_MM;

            if self.is_wall_at(
                detection_x_index as usize,
                detection_y_index as usize,
                detection_x_offset,
                detection_y_offset,
            )? {
                return Ok((
                    distance,
                    scale_line::<R>(sensor_x, sensor_y, detection_x, detection_y),
                ));
            }
        }

        for i in 1..80 {
            let distance = 100 + i * 5;
            let detection_x = sensor_x + distance as f64 * cos;
            let detection_y = sensor_y + distance as f64 * sin;

            let detection_x_index = detection_x as i32 / CELL_SIZE_MM;
            let detection_y_index = detection_y as i32 / CELL_SIZE_MM;

            let detection_x_offset = detection_x as i32 % CELL_SIZE_MM;
            let detection_y_offset = detection_y as i32 % CELL_SIZE_MM;

            if self.is_wall_at(
                detection_x_index as usize,
                detection_y_index as usize,
                detection_x_offset,
                detection_y_offset,
            )? {
                return Ok((
                    distance,
                    scale_line::<R>(sensor_x, sensor_y, detection_x, detection_y),
                ));
            }
        }

        Ok((-1, scale_line::<R>(sensor_x, sensor_y, sensor_x, sensor_y)))
    }

    pub fn front_left(&self, runner_position: &Position<R>) -> Result<(i32, Line)> {
        self.estimate_measured_distance::<FL>(runner_position)
    }

    pub fn front_right(&self, runner_position: &Position<R>) -> Result<(i32, Line)> {
        self.estimate_measured_distance::<FR>(runner_position)
    }

    pub fn diagonal_left(&self, runner_position: &Position<R>) -> Result<(i32, Line)> {
        self.estimate_measured_distance::<DL>(runner_position)
    }

    pub fn diagonal_right(&self, runner_position: &Position<R>) -> Result<(i32, Line)> {
        self.estimate_measured_distance::<DR>(runner_position)
    }
}

pub struct DistanceSensorFrontLeft;
pub struct DistanceSensorFrontRight;
pub struct DistanceSensorDiagonalLeft;
pub struct DistanceSensorDiagonalRight;

impl DistanceSensor for DistanceSensorFrontLeft {
    fn alpha() -> Angle {
        Angle::degrees(0.0)
    }

    fn position_x_offset() -> Millimeters {
        28.0
    }

    fn position_y_offset() -> Millimeters {
        30.0
    }
}

impl DistanceSensor for DistanceSensorFrontRight {
    fn alpha() -> Angle {
        Angle::degrees(0.0)
    }

    fn position_x_offset() -> Millimeters {
        -28.0
    }

    fn position_y_offset() -> Millimeters {
        30.0
    }
}

impl DistanceSensor for DistanceSensorDiagonalLeft {
    fn alpha() -> Angle {
        Angle::degrees(60.0)
    }

    fn position_x_offset() -> Millimeters {
        20.0
    }

    fn position_y_offset() -> Millimeters {
        33.0
    }
}

impl DistanceSensor for DistanceSensorDiagonalRight {
    fn alpha() -> Angle {
        Angle::degrees(-60.0)
    }

    fn position_x_offset() -> Millimeters {
        -20.0
    }

    fn position_y_offset() -> Millimeters {
        33.0
    }
}

#[derive(Clone)]
pub struct DistanceSensorsReading {
    pub fl: i32,
    pub fr: i32,
    pub dl: i32,
    pub dr: i32,
    pub fl_beam: Line,
    pub fr_beam: Line,
    pub dl_beam: Line,
    pub dr_beam: Line,
}

impl DistanceSensorsReading {
    pub fn new() -> Self {
        Self {
            fl: -1,
            fr: -1,
            dl: -1,
            dr: -1,
            fl_beam: line_!([-1, -1], [-1, -1]),
            fr_beam: line_!([-1, -1], [-1, -1]),
            dl_beam: line_!([-1, -1], [-1, -1]),
            dr_beam: line_!([-1, -1], [-1, -1]),
        }
    }
}

impl Render for DistanceSensorsReading {
    fn draw<C>(
        &self,
        s: &mut pix_engine::state::PixState,
        primary_color: C,
        secondary_color: C,
    ) -> Result<()>
    where
        C: Into<Option<pix_engine::prelude::Color>> + std::marker::Copy,
    {
        s.stroke(secondary_color);
        s.fill(primary_color);

        s.line(self.fl_beam)?;
        s.line(self.fr_beam)?;
        s.line(self.dl_beam)?;
        s.line(self.dr_beam)
    }
}

fn scale_line<const R: usize>(x1: f64, y1: f64, x2: f64, y2: f64) -> Line {
    let x1 = (x1 as i32 + WALL_WIDTH_MM / 2) / RATIO_VIS_MM;

    let y1 = (R as i32 * CELL_SIZE_MM - y1 as i32 + WALL_WIDTH_MM / 2) / RATIO_VIS_MM;

    let x2 = (x2 as i32 + WALL_WIDTH_MM / 2) / RATIO_VIS_MM;

    let y2 = (R as i32 * CELL_SIZE_MM - y2 as i32 + WALL_WIDTH_MM / 2) / RATIO_VIS_MM;

    line_!([x1, y1], [x2, y2])
}
