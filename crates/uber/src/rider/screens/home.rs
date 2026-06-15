use fission::prelude::*;
use crate::shared::state::{AppState, Page};

#[fission_component]
#[derive(Clone)]
pub struct HomeScreen {}

#[fission_reducer(GoToSearch)]
fn navigate_search(state: &mut AppState) {
    state.current_page = Page::Search;
}

#[fission_reducer(Logout)]
fn handle_logout(state: &mut AppState) {
    state.auth.is_authenticated = false;
    state.current_page = Page::Splash;
}

    impl From<HomeScreen> for Widget {
        fn from(_screen: HomeScreen) -> Widget {
            let (ctx, _) = fission::build::current::<AppState>();
            
            let on_search = ctx.bind(GoToSearch, reduce!(navigate_search));
            let on_logout = ctx.bind(Logout, reduce!(handle_logout));
    
            // Foreground overlay
            let overlay = Container::new(VStack {
                spacing: Some(16.0),
                children: widgets![
                    // Top Action Bar
                    Widget::from(HStack {
                        spacing: Some(16.0),
                        children: widgets![
                            Widget::from(Container::new(
                                Button {
                                    on_press: Some(on_logout),
                                    child: Some(Widget::from(Text::new("☰").size(24.0).color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 }))),
                                    ..Default::default()
                                }
                            ).bg(fission::op::Color { r: 255, g: 255, b: 255, a: 255 }).border_radius(24.0).padding_all(8.0)),
                            Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                        ],
                        ..Default::default()
                    }),
                    Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                    // Bottom Sheet / Search Box
                    Widget::from(Container::new(VStack {
                        spacing: Some(16.0),
                        children: widgets![
                            Widget::from(Text::new("Good morning, Ola").size(24.0).weight(700)),
                            Widget::from(Container::new(
                                Button {
                                    on_press: Some(on_search),
                                    child: Some(
                                        Widget::from(HStack {
                                            spacing: Some(12.0),
                                            children: widgets![
                                                Widget::from(Text::new("🔍").size(20.0)),
                                                Widget::from(Text::new("Where to?").size(20.0).color(fission::op::Color { r: 50, g: 50, b: 50, a: 255 }).weight(700)),
                                                Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                                                Widget::from(Container::new(Text::new("📍").size(16.0)).bg(fission::op::Color { r: 230, g: 230, b: 230, a: 255 }).border_radius(16.0).padding_all(8.0)),
                                            ],
                                            ..Default::default()
                                        })
                                    ),
                                    ..Default::default()
                                }
                            ).bg(fission::op::Color { r: 240, g: 240, b: 240, a: 255 }).border_radius(24.0).padding_all(16.0))
                        ],
                        ..Default::default()
                    }).bg(fission::op::Color { r: 255, g: 255, b: 255, a: 255 }).border_radius(32.0).padding_all(24.0).shadow(fission::op::BoxShadow {
                        color: fission::op::Color { r: 0, g: 0, b: 0, a: 40 },
                        blur_radius: 24.0,
                        offset: (0.0, -4.0),
                    }))
                ],
                ..Default::default()
            })
            .flex_grow(1.0);
    
            // Map Background
            let map = crate::rider::widgets::map_view::MapView {
                lat: 37.7749,
                lng: -122.4194,
            };
                
            Widget::from(ZStack {
                children: widgets![
                    Widget::from(Positioned {
                        top: Some(0.0),
                        bottom: Some(0.0),
                        left: Some(0.0),
                        right: Some(0.0),
                        child: Some(Widget::from(map)),
                        ..Default::default()
                    }),
                    Widget::from(Positioned {
                        top: Some(0.0),
                        bottom: Some(0.0),
                        left: Some(0.0),
                        right: Some(0.0),
                        child: Some(Widget::from(overlay)),
                        ..Default::default()
                    })
                ],
                ..Default::default()
            })
        }
    }
