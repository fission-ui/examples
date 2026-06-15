use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct TripHistoryScreen {}

#[fission_reducer(GoBackToProfileFromHistory)]
fn navigate_profile(state: &mut AppState) {
    state.current_page = Page::Profile;
}

impl From<TripHistoryScreen> for Widget {
    fn from(_screen: TripHistoryScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackToProfileFromHistory, reduce!(navigate_profile));

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
                        Text::new("Trip History").size(24.0)
                    ],
                    ..Default::default()
                },
                
                // History List Placeholder
                Container::new(Column {
                    gap: Some(16.0),
                    children: widgets![
                        Container::new(Column {
                            gap: Some(4.0),
                            children: widgets![
                                Text::new("Yesterday, 2:30 PM").size(14.0),
                                Text::new("Airport -> Downtown").size(16.0),
                                Text::new("$24.50").size(16.0)
                            ],
                            ..Default::default()
                        })
                        .padding_all(12.0),
                        
                        Container::new(Column {
                            gap: Some(4.0),
                            children: widgets![
                                Text::new("Mon, 8:00 AM").size(14.0),
                                Text::new("Home -> Office").size(16.0),
                                Text::new("$12.00").size(16.0)
                            ],
                            ..Default::default()
                        })
                        .padding_all(12.0)
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
