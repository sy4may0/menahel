use ratatui::{
    layout::{Margin, Rect},
    widgets::{Block, BorderType},
    Frame,
    text::{Line},
    style::{Style, Color, Stylize},
    crossterm::event::KeyEvent,
};
use anyhow::Result;
use crate::client::component::Component;

use crate::client::event::{
    AppEvent,
    Tx,
};
use crate::client::ui::PaneId;

pub struct Pane {
    pub title: String,
    pub sender: Tx,
    pub child: Option<Box<dyn Component>>,
    pub focus: bool,
    pub pane_id: PaneId,
}

impl Pane {
    pub fn new(
        title: String, 
        sender: Tx, 
        initial_focus: bool,
        pane_id: PaneId
    ) -> Self {
        Self {
            title,
            sender,
            child: None,
            focus: initial_focus,
            pane_id,
        }
    }
}

impl Pane {
    pub fn set_child(&mut self, child: Box<dyn Component>) {
        self.child = Some(child);
    }
}

impl Component for Pane {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(child) = &mut self.child {
            let inner_area = area.inner(
                Margin {
                    vertical: 1,
                    horizontal: 1,
                }
            );
            child.draw(frame, inner_area)?;
        }

        let block = match self.focus {
            true => {
                let t = Line::from(format!("*{}*", self.title.clone()));
                Block::bordered()
                .title(t)
                .border_type(BorderType::Plain)
                .border_style(Style::new().fg(Color::Green).bold())
                .title_style(Style::new().fg(Color::Green).bold())
            }
            false => {
                let t = Line::from(self.title.clone());
                Block::bordered()
                .title(t)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Gray))
                .title_style(Style::default().fg(Color::Gray))
            }
        };

        frame.render_widget(
            block,
            area,
        );
        Ok(())
    }

    fn handle_app_event(&mut self, event: &AppEvent) -> Result<()> {
        match event {
            AppEvent::FocusPane(pane_id) => {
                if *pane_id == self.pane_id {
                    self.set_focus(true);
                } else {
                    self.set_focus(false);
                }
            }
            _ => {}
        }

        if let Some(child) = &mut self.child {
            child.handle_app_event(event)?;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Result<()> {
        if let Some(child) = &mut self.child {
            child.handle_key_event(key_event)?;
        }
        Ok(())
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
        if let Some(child) = &mut self.child {
            child.set_focus(focus);
        }
    }
}

