use crate::error::{AppError, AppResult};
use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let _sender = sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = _sender.closed() => {
                        break;
                    }
                    _ = tick_delay => {
                        if let Err(err) = _sender.send(Event::Tick) {
                            log::error!("Failed to send Tick event: {}", err);
                            break;
                        }
                    }
                    Some(result) = crossterm_event => {
                        match result {
                            Ok(evt) => {
                                if let Err(err) = handle_event(evt, &_sender) {
                                    log::error!("Failed to handle event: {}", err);
                                    break;
                                }
                            }
                            Err(err) => {
                                log::error!("Error reading event: {}", err);
                                break;
                            }
                        }
                    }
                };
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub async fn next(&mut self) -> AppResult<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(AppError::EventStreamClosed)
    }
}

fn handle_event(evt: CrosstermEvent, sender: &mpsc::UnboundedSender<Event>) -> AppResult<()> {
    match evt {
        CrosstermEvent::Key(key) => {
            if key.kind == crossterm::event::KeyEventKind::Press {
                sender
                    .send(Event::Key(key))
                    .map_err(|e| AppError::SendError(e.to_string()))?;
            }
        }
        CrosstermEvent::Mouse(mouse) => {
            sender
                .send(Event::Mouse(mouse))
                .map_err(|e| AppError::SendError(e.to_string()))?;
        }
        CrosstermEvent::Resize(x, y) => {
            sender
                .send(Event::Resize(x, y))
                .map_err(|e| AppError::SendError(e.to_string()))?;
        }
        CrosstermEvent::FocusLost | CrosstermEvent::FocusGained | CrosstermEvent::Paste(_) => {}
    }
    Ok(())
}
