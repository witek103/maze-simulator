use anyhow::Result;
use pix_engine::{prelude::Color, rect, state::PixState};

use crate::{
    engine::Render,
    simulator::{APP_HEIGHT, APP_WIDTH, PANEL_WIDTH},
};

pub const PANEL_Y_OFFSET: i32 = 0;
pub const PANEL_X_OFFSET: i32 = APP_WIDTH as i32 - PANEL_WIDTH;

#[derive(Debug)]
pub enum Button {
    Reset,
    Button1,
    Button2,
    Button3,
    Button4,
}

pub struct SimPanel {}

impl SimPanel {
    pub fn button_pressed(&self, button: Button) {
        print!("Pressed: {:?}\n", button);
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
            self.button_pressed(Button::Reset);
        }
        s.same_line(None);
        if s.button("BTN1")? {
            self.button_pressed(Button::Button1);
        }

        s.same_line(None);
        if s.button("BTN2")? {
            self.button_pressed(Button::Button2);
        }

        s.same_line(None);
        if s.button("BTN3")? {
            self.button_pressed(Button::Button3);
        }

        s.same_line(None);
        if s.button("BTN4")? {
            self.button_pressed(Button::Button4);
        }

        Ok(())
    }
}
