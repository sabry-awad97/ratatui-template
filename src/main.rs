use crate::{app::App, error::AppResult};

pub mod app;
pub mod error;
pub mod event;
pub mod state;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut terminal = ratatui::init();

    App::new().run(&mut terminal).await?;

    ratatui::restore();
    Ok(())
}
