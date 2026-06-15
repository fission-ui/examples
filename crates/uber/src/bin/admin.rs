//! Admin Panel — Desktop entry point

use fission::prelude::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Admin Panel");

    DesktopApp::<uber::shared::state::AppState, _>::new(uber::admin::app::AdminApp {})
        .run()?;

    Ok(())
}
