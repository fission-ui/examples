use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct DashboardScreen {}

#[fission_reducer(GoToUsers)]
fn navigate_users(state: &mut AppState) {
    state.current_page = Page::AdminUsers;
}

#[fission_reducer(GoToDrivers)]
fn navigate_drivers(state: &mut AppState) {
    state.current_page = Page::AdminDrivers;
}

#[fission_reducer(GoToTrips)]
fn navigate_trips(state: &mut AppState) {
    state.current_page = Page::AdminTrips;
}

impl From<DashboardScreen> for Widget {
    fn from(_screen: DashboardScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_users = ctx.bind(GoToUsers, reduce!(navigate_users));
        let on_drivers = ctx.bind(GoToDrivers, reduce!(navigate_drivers));
        let on_trips = ctx.bind(GoToTrips, reduce!(navigate_trips));

        Container::new(Column {
            gap: Some(24.0),
            children: widgets![
                Text::new("Admin Dashboard").size(32.0),
                
                // Key Metrics
                Row {
                    gap: Some(24.0),
                    children: widgets![
                        Container::new(Column {
                            gap: Some(8.0),
                            children: widgets![
                                Text::new("Active Users").size(14.0),
                                Text::new("12,450").size(24.0)
                            ],
                            ..Default::default()
                        }).padding_all(16.0),
                        Container::new(Column {
                            gap: Some(8.0),
                            children: widgets![
                                Text::new("Online Drivers").size(14.0),
                                Text::new("842").size(24.0)
                            ],
                            ..Default::default()
                        }).padding_all(16.0),
                        Container::new(Column {
                            gap: Some(8.0),
                            children: widgets![
                                Text::new("Live Trips").size(14.0),
                                Text::new("315").size(24.0)
                            ],
                            ..Default::default()
                        }).padding_all(16.0)
                    ],
                    ..Default::default()
                },
                
                // Navigation
                Row {
                    gap: Some(16.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_users),
                            child: Some(Text::new("Manage Users").into()),
                            ..Default::default()
                        },
                        Button {
                            on_press: Some(on_drivers),
                            child: Some(Text::new("Manage Drivers").into()),
                            ..Default::default()
                        },
                        Button {
                            on_press: Some(on_trips),
                            child: Some(Text::new("Monitor Trips").into()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
