use std::sync::{Arc, Mutex};

use anyhow::Result;
use pix_engine::{prelude::Color, rect, state::PixState};

use crate::{
    communication::ButtonsState,
    engine::Render,
    simulator::{APP_HEIGHT, APP_WIDTH, PANEL_WIDTH},
};

pub const PANEL_Y_OFFSET: i32 = 0;
pub const PANEL_X_OFFSET: i32 = APP_WIDTH as i32 - PANEL_WIDTH;

pub struct SimPanel {
    buttons: Arc<Mutex<ButtonsState>>,
}

impl SimPanel {
    pub fn new(buttons: Arc<Mutex<ButtonsState>>) -> Self {
        Self { buttons }
    }

    pub fn button_pressed(&self, button: ButtonsState) {
        let mut buttons_state = self.buttons.lock().unwrap();

        buttons_state.set(button, true);
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

        s.set_cursor_pos([PANEL_X_OFFSET + 10, 5]);
        s.fill(Color::BLACK);
        s.stroke(None);

        s.text(format!("Runner buttons:"))?;

        s.set_cursor_pos([PANEL_X_OFFSET + 10, 30]);
        s.fill(Color::WHITE);
        s.stroke(None);

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
