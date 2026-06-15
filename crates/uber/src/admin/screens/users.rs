use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct UsersScreen {}

#[fission_reducer(GoBackToDashboard)]
fn navigate_dashboard(state: &mut AppState) {
    state.current_page = Page::AdminDashboard;
}

impl From<UsersScreen> for Widget {
    fn from(_screen: UsersScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackToDashboard, reduce!(navigate_dashboard));

        Container::new(Column {
            gap: Some(24.0),
            children: widgets![
                Row {
                    gap: Some(16.0),
                    children: widgets![
                        Button {
                            on_press: Some(on_back),
                            child: Some(Text::new("< Back").size(16.0).into()),
                            ..Default::default()
                        },
                        Text::new("Manage Users").size(32.0)
                    ],
                    ..Default::default()
                },
                
                // Users Table Placeholder
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("User ID | Name | Status | Actions").size(16.0),
                        Text::new("-------------------------------------------------").size(16.0),
                        Text::new("USR_001 | Jane Doe | Active | [Suspend] [View]").size(14.0),
                        Text::new("USR_002 | Mark Lee | Active | [Suspend] [View]").size(14.0)
                    ],
                    ..Default::default()
                })
                .padding_all(24.0)
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        .into()
    }
}
