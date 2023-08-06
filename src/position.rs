use anyhow::Result;
use core::{
    f64::consts::PI,
    ops::{Add, Div, Sub},
};
use libm::{cos, fabs, sin};
use pix_engine::{
    prelude::{AngleMode, Color},
    state::PixState,
};

use crate::{
    engine::Render,
    simulator::{CELL_SIZE_MM, RATIO_VIS_MM, WALL_WIDTH_MM},
};

pub const RUNNER_SIZE_MM: f64 = 64.0;

const RUNNER_SHAPE_VERTEXES: [[f64; 2]; 6] = [
    [-(RUNNER_SIZE_MM / 2.0), RUNNER_SIZE_MM / 2.0],
    [RUNNER_SIZE_MM / 2.0, RUNNER_SIZE_MM / 2.0],
    [RUNNER_SIZE_MM / 1.5, RUNNER_SIZE_MM / 5.0],
    [RUNNER_SIZE_MM / 1.5, -(RUNNER_SIZE_MM / 5.0)],
    [RUNNER_SIZE_MM / 2.0, -(RUNNER_SIZE_MM / 2.0)],
    [-(RUNNER_SIZE_MM / 2.0), -(RUNNER_SIZE_MM / 2.0)],
];

pub type Millimeters = f64;
pub type Radians = f64;
pub type Degrees = f64;

#[derive(Copy, Clone)]
pub struct Angle {
    value: f64,
}

#[derive(Clone)]
pub struct Position<const R: usize> {
    pub x: Millimeters,
    pub y: Millimeters,
    pub theta: Angle,
}

impl Angle {
    pub fn radians(value: Radians) -> Self {
        Self { value }.normalize()
    }

    pub fn degrees(value: Degrees) -> Self {
        Self::radians(value.to_radians())
    }

    #[allow(dead_code)]
    pub fn as_radians(&self) -> Radians {
        self.value
    }

    pub fn as_degrees(&self) -> Degrees {
        self.value.to_degrees()
    }

    #[allow(dead_code)]
    pub fn abs(&self) -> Self {
        Self {
            value: fabs(self.value),
        }
    }

    #[allow(dead_code)]
    pub fn cos(&self) -> f64 {
        cos(self.as_radians())
    }

    #[allow(dead_code)]
    pub fn sin(&self) -> f64 {
        sin(self.as_radians())
    }

    #[allow(dead_code)]
    pub fn is_within(&self, other: &Angle, difference: Angle) -> bool {
        (self.clone() - other.clone()).abs().as_radians() < difference.as_radians()
    }

    fn normalize(self) -> Self {
        let value = self.value % (2.0 * PI);

        let value = if value > PI {
            value - 2.0 * PI
        } else if value < -PI {
            value + 2.0 * PI
        } else {
            value
        };

        Self { value }
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let value = self.value + rhs.value;
        Self { value }.normalize()
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let value = self.value - rhs.value;
        Self { value }.normalize()
    }
}

impl Div<f64> for Angle {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value / rhs,
        }
    }
}

impl<const R: usize> Position<R> {
    pub fn new(x: Millimeters, y: Millimeters, theta: Angle) -> Self {
        Self { x, y, theta }
    }
}

impl<const R: usize> Render for Position<R> {
    fn draw<T>(&self, s: &mut PixState, primary_color: T, secondary_color: T) -> Result<()>
    where
        T: Into<Option<Color>>,
    {
        s.stroke(secondary_color);
        s.fill(primary_color);
        s.angle_mode(AngleMode::Degrees);

        s.wireframe(
            RUNNER_SHAPE_VERTEXES,
            [
                (self.x as i32 + WALL_WIDTH_MM / 2) / RATIO_VIS_MM,
                (R as i32 * CELL_SIZE_MM - self.y as i32 + WALL_WIDTH_MM / 2) / RATIO_VIS_MM,
            ],
            360.0 - self.theta.as_degrees(),
            1.0 / RATIO_VIS_MM as f64,
        )?;

        Ok(())
    }
}
