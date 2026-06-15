use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct SplashScreen {}

#[fission_reducer(GoToLogin)]
fn navigate_home(state: &mut AppState) {
    state.current_page = Page::Login;
}

impl From<SplashScreen> for Widget {
    fn from(_screen: SplashScreen) -> Widget {
        let (ctx, _) = fission::build::current::<AppState>();
        let on_enter = ctx.bind(GoToLogin, reduce!(navigate_home));

        Container::new(VStack {
            spacing: Some(24.0),
            children: widgets![
                Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                Widget::from(Center {
                    child: Widget::from(Text::new("Uber").size(64.0))
                }),
                Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                Widget::from(Center {
                    child: Widget::from(Button {
                        on_press: Some(on_enter),
                        child: Some(Widget::from(Text::new("Get Started").size(20.0))),
                        ..Default::default()
                    })
                }),
                Widget::from(Spacer { height: Some(48.0), ..Default::default() }),
            ],
            ..Default::default()
        })
        .padding_all(32.0)
        // .bg(Color::BLACK)
        .flex_grow(1.0)
        .into()
    }
}
