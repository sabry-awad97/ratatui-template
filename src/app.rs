use crate::{
    error::{AppError, AppResult},
    event::{Event, EventHandler},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    DefaultTerminal,
};

#[derive(Debug)]
pub struct App {
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> AppResult<()> {
        let mut events = EventHandler::new(250);

        while self.running {
            terminal
                .draw(|frame| frame.render_widget(&*self, frame.area()))
                .map_err(|e| AppError::DrawError(e.to_string()))?;
            match events.next().await? {
                Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                Event::Tick => {}
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }

        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> AppResult<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.quit();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    self.quit();
                }
            }
            KeyCode::Right => {}
            KeyCode::Left => {}
            _ => {}
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Sabry")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        let title = Paragraph::new("Hello World")
            .alignment(Alignment::Center)
            .block(Block::default());

        title.render(inner, buf);
    }
}
