//! Root application component and state for the Driver App.

use fission::prelude::*;
use crate::shared::state::{AppState, Page};

use crate::driver::screens::{
    splash::SplashScreen,
    home::HomeScreen,
    auth::{login::LoginScreen, signup::SignupScreen},
    ride_requests::RideRequestsScreen,
    active_trip::ActiveTripScreen,
    earnings::EarningsScreen,
    profile::ProfileScreen,
};

#[fission_component]
#[derive(Clone)]
pub struct DriverApp {}

impl From<DriverApp> for Widget {
    fn from(_app: DriverApp) -> Widget {
        let (_ctx, view) = fission::build::current::<AppState>();
        let current_page = view.state().current_page.clone();

        let child: Widget = match current_page {
            Page::Splash => SplashScreen {}.into(),
            Page::Login => LoginScreen {}.into(),
            Page::Signup => SignupScreen {}.into(),
            Page::Home => HomeScreen {}.into(),
            Page::RideRequest => RideRequestsScreen {}.into(), // Map to ride requests instead of rider booking
            Page::TripTracking => ActiveTripScreen {}.into(), // Map to active trip tracking
            Page::Profile => ProfileScreen {}.into(),
            Page::Earnings => EarningsScreen {}.into(),
            _ => Container::new(
                Text::new("Screen not implemented").size(24.0)
            ).into(),
        };

        child
    }
}
