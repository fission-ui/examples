use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct SearchScreen {}

#[fission_reducer(GoBack)]
fn navigate_back(state: &mut AppState) {
    state.current_page = Page::Home;
}

#[fission_reducer(SelectDestination)]
fn handle_destination_select(state: &mut AppState) {
    state.current_page = Page::RideRequest;
}

impl From<SearchScreen> for Widget {
    fn from(_screen: SearchScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBack, reduce!(navigate_back));
        let on_select = ctx.bind(SelectDestination, reduce!(handle_destination_select));

        Container::new(Column {
            gap: Some(16.0),
            children: widgets![
                // Header row
                Row {
                    gap: Some(16.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_back),
                            child: Some(Text::new("< Back").size(16.0).into()),
                            ..Default::default()
                        },
                        Text::new("Search Destination").size(24.0)
                    ],
                    ..Default::default()
                },
                
                // Search Input
                Container::new(Text::new("Enter destination...").size(16.0))
                    .padding_all(16.0),

                // Mock Predictions
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_select.clone()),
                            child: Some(Text::new("Airport").size(16.0).into()),
                            ..Default::default()
                        },
                        Button {
                            on_press: Some(on_select.clone()),
                            child: Some(Text::new("Train Station").size(16.0).into()),
                            ..Default::default()
                        },
                        Button {
                            on_press: Some(on_select),
                            child: Some(Text::new("Shopping Mall").size(16.0).into()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
                .padding_all(8.0)
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
