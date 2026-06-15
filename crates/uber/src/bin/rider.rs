//! Rider App — Desktop entry point

use fission::prelude::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Rider App");

    DesktopApp::<uber::shared::state::AppState, _>::new(uber::rider::app::RiderApp {})
        .run()?;

    Ok(())
}
