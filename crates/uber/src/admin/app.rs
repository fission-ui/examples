//! Root application component and state for the Admin Panel.

use fission::prelude::*;
use crate::shared::state::{AppState, Page};

use crate::admin::screens::{
    login::LoginScreen,
    dashboard::DashboardScreen,
    users::UsersScreen,
    drivers::DriversScreen,
    trips::TripsScreen,
};

#[fission_component]
#[derive(Clone)]
pub struct AdminApp {}

impl From<AdminApp> for Widget {
    fn from(_app: AdminApp) -> Widget {
        let (_ctx, view) = fission::build::current::<AppState>();
        
        // Let's use Splash as the initial state, but map it to Login for Admin Panel
        let current_page = match view.state().current_page.clone() {
            Page::Splash => Page::Login,
            other => other,
        };

        let child: Widget = match current_page {
            Page::Login => LoginScreen {}.into(),
            Page::AdminDashboard => DashboardScreen {}.into(),
            Page::AdminUsers => UsersScreen {}.into(),
            Page::AdminDrivers => DriversScreen {}.into(),
            Page::AdminTrips => TripsScreen {}.into(),
            _ => Container::new(
                Text::new("Screen not implemented").size(24.0)
            ).into(),
        };

        child
    }
}
