//! Root application component and state for the Rider App.

use fission::prelude::*;
use crate::shared::state::{AppState, Page};

use crate::rider::screens::{
    splash::SplashScreen,
    home::HomeScreen,
    search::SearchScreen,
    ride_request::RideRequestScreen,
    trip_tracking::TripTrackingScreen,
    profile::ProfileScreen,
    trip_history::TripHistoryScreen,
    payment::PaymentScreen,
    auth::{login::LoginScreen, signup::SignupScreen},
};

#[fission_component]
#[derive(Clone)]
pub struct RiderApp {}

impl From<RiderApp> for Widget {
    fn from(_app: RiderApp) -> Widget {
        let (_ctx, view) = fission::build::current::<AppState>();
        let current_page = view.state().current_page.clone();

        let child: Widget = match current_page {
            Page::Splash => SplashScreen {}.into(),
            Page::Login => LoginScreen {}.into(),
            Page::Signup => SignupScreen {}.into(),
            Page::Home => HomeScreen {}.into(),
            Page::Search => SearchScreen {}.into(),
            Page::RideRequest => RideRequestScreen {}.into(),
            Page::TripTracking => TripTrackingScreen {}.into(),
            Page::Profile => ProfileScreen {}.into(),
            Page::TripHistory => TripHistoryScreen {}.into(),
            Page::Payment => PaymentScreen {}.into(),
            _ => Container::new(
                Text::new("Screen not implemented").size(24.0)
            ).into(),
        };

        child
    }
}
