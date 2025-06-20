use ratatui::{
    layout::Rect,
    Frame,
    crossterm::event::KeyEvent,
};

use crate::client::event::AppEvent;
use color_eyre::Result;

pub trait Component {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;

    #[allow(unused_variables)]
    fn handle_event(&mut self, event: AppEvent) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        Ok(())
    }

    fn tick(&self) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn set_focus(&mut self, focus: bool) {
        {}
    } 
}