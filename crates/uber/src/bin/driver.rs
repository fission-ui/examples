//! Driver App — Desktop entry point

use fission::prelude::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Driver App");

    DesktopApp::<uber::shared::state::AppState, _>::new(uber::driver::app::DriverApp {})
        .run()?;

    Ok(())
}
