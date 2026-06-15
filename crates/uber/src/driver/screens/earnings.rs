use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct EarningsScreen {}

#[fission_reducer(GoBackFromEarnings)]
fn navigate_home(state: &mut AppState) {
    state.current_page = Page::Home;
}

impl From<EarningsScreen> for Widget {
    fn from(_screen: EarningsScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackFromEarnings, reduce!(navigate_home));

        Container::new(Column {
            gap: Some(16.0),
            children: widgets![
                Row {
                    gap: Some(16.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_back),
                            child: Some(Text::new("< Back").size(16.0).into()),
                            ..Default::default()
                        },
                        Text::new("Earnings").size(24.0)
                    ],
                    ..Default::default()
                },
                
                // Weekly Earnings
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("This Week").size(18.0),
                        Text::new("$420.50").size(32.0),
                        Text::new("32 Trips").size(16.0)
                    ],
                    ..Default::default()
                })
                .padding_all(24.0),

                // Chart Placeholder
                Container::new(Text::new("Earnings Chart Placeholder").size(16.0))
                    .padding_all(48.0)
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
