use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct PaymentScreen {}

#[fission_reducer(GoBackToProfileFromPayment)]
fn navigate_profile(state: &mut AppState) {
    state.current_page = Page::Profile;
}

impl From<PaymentScreen> for Widget {
    fn from(_screen: PaymentScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        
        let on_back = ctx.bind(GoBackToProfileFromPayment, reduce!(navigate_profile));

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
                        Text::new("Payment Methods").size(24.0)
                    ],
                    ..Default::default()
                },
                
                // Payment Methods List
                Container::new(Column {
                    gap: Some(16.0),
                    children: widgets![
                        Container::new(Text::new("Visa ending in 4242").size(18.0))
                            .padding_all(16.0),
                        Container::new(Text::new("Apple Pay").size(18.0))
                            .padding_all(16.0),
                        
                        Button {
                            on_press: None,
                            child: Some(Text::new("+ Add Payment Method").size(16.0).into()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
                .padding_all(16.0)
            ],
            ..Default::default()
        })
        .padding_all(16.0)
        .into()
    }
}
