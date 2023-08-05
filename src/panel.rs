use std::sync::{Arc, Mutex};

use anyhow::Result;
use pix_engine::{prelude::Color, rect, state::PixState};

use crate::{
    communication::ButtonsState,
    distance_sensors::DistanceSensorsReading,
    engine::Render,
    simulator::{APP_HEIGHT, APP_WIDTH, PANEL_WIDTH},
};

pub const PANEL_Y_OFFSET: i32 = 0;
pub const PANEL_X_OFFSET: i32 = APP_WIDTH as i32 - PANEL_WIDTH;

pub struct SimPanel {
    buttons: Arc<Mutex<ButtonsState>>,
    distance_sensors: Arc<Mutex<DistanceSensorsReading>>,
}

impl SimPanel {
    pub fn new(
        buttons: Arc<Mutex<ButtonsState>>,
        distance_sensors: Arc<Mutex<DistanceSensorsReading>>,
    ) -> Self {
        Self {
            buttons,
            distance_sensors,
        }
    }

    pub fn button_pressed(&self, button: ButtonsState) {
        let mut buttons_state = self.buttons.lock().unwrap();

        buttons_state.set(button, true);
    }

    fn draw_sensor_readings(&self, s: &mut PixState) -> Result<()> {
        let x_offset = PANEL_X_OFFSET + 10;
        let y_offset = 100;
        let x_padding = 40;
        let y_padding = 20;
        let bar_width = 10;

        let distance_sensors = self.distance_sensors.lock().unwrap().clone();

        s.set_cursor_pos([x_offset, y_offset]);
        s.fill(Color::BLACK);
        s.stroke(None);

        s.text(format!("DL:"))?;

        s.set_cursor_pos([x_offset + x_padding, y_offset]);

        s.text(format!("{}", distance_sensors.dl))?;

        s.set_cursor_pos([x_offset, y_offset + y_padding]);

        s.text(format!("FL:"))?;

        s.set_cursor_pos([x_offset + x_padding, y_offset + y_padding]);

        s.text(format!("{}", distance_sensors.fl))?;

        s.set_cursor_pos([x_offset, y_offset + y_padding * 2]);

        s.text(format!("FR:"))?;

        s.set_cursor_pos([x_offset + x_padding, y_offset + y_padding * 2]);

        s.text(format!("{}", distance_sensors.fr))?;

        s.set_cursor_pos([x_offset, y_offset + y_padding * 3]);

        s.text(format!("DR:"))?;

        s.set_cursor_pos([x_offset + x_padding, y_offset + y_padding * 3]);

        s.text(format!("{}", distance_sensors.dr))?;

        s.rect(rect![
            x_offset + x_padding * 2,
            y_offset + 5,
            distance_sensors.dl / 2,
            bar_width,
        ])?;

        s.rect(rect![
            x_offset + x_padding * 2,
            y_offset + y_padding + 5,
            distance_sensors.fl / 2,
            bar_width,
        ])?;

        s.rect(rect![
            x_offset + x_padding * 2,
            y_offset + y_padding * 2 + 5,
            distance_sensors.fr / 2,
            bar_width,
        ])?;

        s.rect(rect![
            x_offset + x_padding * 2,
            y_offset + y_padding * 3 + 5,
            distance_sensors.dr / 2,
            bar_width,
        ])?;

        Ok(())
    }

    fn draw_buttons(&self, s: &mut PixState) -> Result<()> {
        s.set_cursor_pos([PANEL_X_OFFSET + 10, 5]);
        s.fill(Color::BLACK);
        s.stroke(None);

        s.text(format!("Runner buttons:"))?;

        s.set_cursor_pos([PANEL_X_OFFSET + 10, 30]);

        if s.button("Reset")? {
            self.button_pressed(ButtonsState::Reset);
        }

        s.same_line(None);
        if s.button("BTN1")? {
            self.button_pressed(ButtonsState::Button1);
        }

        s.same_line(None);
        if s.button("BTN2")? {
            self.button_pressed(ButtonsState::Button2);
        }

        s.same_line(None);
        if s.button("BTN3")? {
            self.button_pressed(ButtonsState::Button3);
        }

        s.same_line(None);
        if s.button("BTN4")? {
            self.button_pressed(ButtonsState::Button4);
        }

        Ok(())
    }
}

impl Render for SimPanel {
    fn draw<T>(&self, s: &mut PixState, primary_color: T, secondary_color: T) -> Result<()>
    where
        T: Into<Option<Color>>,
    {
        s.stroke(secondary_color);
        s.fill(primary_color);

        s.rect(rect![
            PANEL_X_OFFSET,
            PANEL_Y_OFFSET,
            PANEL_WIDTH,
            APP_HEIGHT as i32,
        ])?;

        self.draw_buttons(s)?;

        self.draw_sensor_readings(s)?;

        Ok(())
    }
}
