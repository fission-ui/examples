use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct DriversScreen {}

#[fission_reducer(GoBackToDashboardFromDrivers)]
fn navigate_dashboard(state: &mut AppState) {
    state.current_page = Page::AdminDashboard;
}

impl From<DriversScreen> for Widget {
    fn from(_screen: DriversScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackToDashboardFromDrivers, reduce!(navigate_dashboard));

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
                        Text::new("Manage Drivers").size(32.0)
                    ],
                    ..Default::default()
                },
                
                // Drivers Table Placeholder
                Container::new(Column {
                    gap: Some(8.0),
                    children: widgets![
                        Text::new("Pending Approvals").size(20.0),
                        Text::new("Driver ID | Name | Documents | Actions").size(16.0),
                        Text::new("-------------------------------------------------").size(16.0),
                        Text::new("DRV_099 | Sarah M. | Pending | [Approve] [Reject]").size(14.0),
                        Text::new("DRV_102 | Alex B.  | Pending | [Approve] [Reject]").size(14.0)
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
