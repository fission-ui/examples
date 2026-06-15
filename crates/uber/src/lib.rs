pub mod shared;
pub mod rider;
pub mod driver;
pub mod admin;


// Default iOS mobile entry point sets the Rider App as the primary app
#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn fission_mobile_main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,wgpu=warn,fission=debug")
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting Fission iOS app for Uber Rider!");

    MobileApp::<rider::app::RiderAppState, _>::new(rider::app::RiderApp {})
        .run()
        .expect("Failed to run mobile app");
}
