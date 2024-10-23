use app::App;
use color_eyre::Result;
use logging::initialize_logging;

mod action;
pub mod app;
mod logging;
pub mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging()?;

    App::new().run().await?;
    Ok(())
}
