pub mod shared;
pub mod rider;
pub mod driver;
pub mod admin;


#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn fission_mobile_main() {
    // We only use standard formatting here because the tracing-subscriber
    // feature for env-filter might not be fully enabled in the workspace.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting Fission iOS app for Uber Rider!");

    fission::prelude::MobileApp::<shared::state::AppState, _>::new(rider::app::RiderApp {})
        .run()
        .expect("Failed to run mobile app");
}
