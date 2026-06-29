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
    
            // Clean, classic Uber Light Theme Overlay
            let overlay = Container::new(VStack {
                children: widgets![
                    // Top Action Bar
                    Widget::from(Container::new(HStack {
                        children: widgets![
                            Widget::from(Container::new(
                                Button {
                                    on_press: Some(on_logout),
                                    child: Some(Widget::from(
                                        fission::widgets::Icon::svg(fission_icons::material::navigation::menu::regular())
                                            .size(28.0)
                                            .color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 })
                                    )),
                                    ..Default::default()
                                }
                            ).bg(fission::op::Color { r: 255, g: 255, b: 255, a: 255 })
                             .border_radius(28.0)
                             .padding_all(12.0)
                             .shadow(fission::op::BoxShadow {
                                 color: fission::op::Color { r: 0, g: 0, b: 0, a: 25 },
                                 blur_radius: 12.0,
                                 offset: (0.0, 4.0),
                             })),
                            Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                        ],
                        ..Default::default()
                    })
                    .padding([20.0, 20.0, 60.0, 0.0])),

                    Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),

                    // Current Location FAB
                    Widget::from(Container::new(HStack {
                        children: widgets![
                            Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                            Widget::from(Container::new(
                                Button {
                                    on_press: None,
                                    child: Some(Widget::from(
                                        fission::widgets::Icon::svg(fission_icons::material::maps::my_location::regular())
                                            .size(24.0)
                                            .color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 })
                                    )),
                                    ..Default::default()
                                }
                            ).bg(fission::op::Color { r: 255, g: 255, b: 255, a: 255 })
                             .border_radius(28.0)
                             .padding_all(14.0)
                             .shadow(fission::op::BoxShadow {
                                 color: fission::op::Color { r: 0, g: 0, b: 0, a: 20 },
                                 blur_radius: 8.0,
                                 offset: (0.0, 2.0),
                             })),
                        ],
                        ..Default::default()
                    }).padding([0.0, 20.0, 0.0, 16.0])),

                    // Bottom Sheet
                    Widget::from(Container::new(VStack {
                        spacing: Some(24.0),
                        children: widgets![
                            // Drag handle
                            Widget::from(HStack {
                                children: widgets![
                                    Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                                    Widget::from(Container::new(Spacer { ..Default::default() })
                                        .bg(fission::op::Color { r: 210, g: 210, b: 210, a: 255 })
                                        .width(48.0)
                                        .height(4.0)
                                        .border_radius(2.0)),
                                    Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                                ],
                                ..Default::default()
                            }),
                            
                            Widget::from(Text::new("Good morning, Ola").size(28.0).weight(700).color(fission::op::Color { r: 10, g: 10, b: 10, a: 255 })),
                            
                            // "Where to?" Box
                            Widget::from(Container::new(
                                Button {
                                    on_press: Some(on_search),
                                    child: Some(
                                        Widget::from(HStack {
                                            spacing: Some(16.0),
                                            children: widgets![
                                                Widget::from(fission::widgets::Icon::svg(fission_icons::material::action::search::regular()).size(28.0).color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 })),
                                                Widget::from(Text::new("Where to?").size(22.0).color(fission::op::Color { r: 10, g: 10, b: 10, a: 255 }).weight(700)),
                                                Widget::from(Spacer { flex_grow: 1.0, ..Default::default() }),
                                                // Time pill
                                                Widget::from(Container::new(
                                                    HStack {
                                                        spacing: Some(4.0),
                                                        children: widgets![
                                                            Widget::from(fission::widgets::Icon::svg(fission_icons::material::action::schedule::regular()).size(14.0).color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 })),
                                                            Widget::from(Text::new("Now").size(14.0).color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 }).weight(600)),
                                                            Widget::from(fission::widgets::Icon::svg(fission_icons::material::navigation::expand_more::regular()).size(14.0).color(fission::op::Color { r: 100, g: 100, b: 100, a: 255 })),
                                                        ],
                                                        ..Default::default()
                                                    }
                                                ).bg(fission::op::Color { r: 255, g: 255, b: 255, a: 255 })
                                                 .border_radius(20.0)
                                                 .padding([12.0, 12.0, 8.0, 8.0])
                                                 .shadow(fission::op::BoxShadow {
                                                     color: fission::op::Color { r: 0, g: 0, b: 0, a: 15 },
                                                     blur_radius: 8.0,
                                                     offset: (0.0, 2.0),
                                                 })),
                                            ],
                                            ..Default::default()
                                        })
                                    ),
                                    ..Default::default()
                                }
                            ).bg(fission::op::Color { r: 238, g: 238, b: 238, a: 255 })
                             .border_radius(24.0)
                             .padding([20.0, 20.0, 16.0, 16.0])),
                            
                            // Suggestions Row
                            Widget::from(HStack {
                                spacing: Some(16.0),
                                children: widgets![
                                    Widget::from(Container::new(VStack {
                                        spacing: Some(12.0),
                                        children: widgets![
                                            Widget::from(Container::new(
                                                fission::widgets::Icon::svg(fission_icons::material::maps::directions_car::regular())
                                                    .size(40.0)
                                                    .color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 })
                                            )
                                                .width(86.0)
                                                .height(80.0)
                                                .bg(fission::op::Color { r: 238, g: 238, b: 238, a: 255 })
                                                .border_radius(16.0)
                                                .padding([23.0, 23.0, 20.0, 20.0])),
                                            Widget::from(Text::new("Ride").size(15.0).color(fission::op::Color { r: 10, g: 10, b: 10, a: 255 }).weight(600)),
                                        ],
                                        ..Default::default()
                                    })),
                                    Widget::from(Container::new(VStack {
                                        spacing: Some(12.0),
                                        children: widgets![
                                            Widget::from(Container::new(
                                                fission::widgets::Icon::svg(fission_icons::material::maps::local_shipping::regular())
                                                    .size(36.0)
                                                    .color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 })
                                            )
                                                .width(86.0)
                                                .height(80.0)
                                                .bg(fission::op::Color { r: 238, g: 238, b: 238, a: 255 })
                                                .border_radius(16.0)
                                                .padding([25.0, 25.0, 22.0, 22.0])),
                                            Widget::from(Text::new("Package").size(15.0).color(fission::op::Color { r: 10, g: 10, b: 10, a: 255 }).weight(600)),
                                        ],
                                        ..Default::default()
                                    })),
                                    Widget::from(Container::new(VStack {
                                        spacing: Some(12.0),
                                        children: widgets![
                                            Widget::from(Container::new(
                                                fission::widgets::Icon::svg(fission_icons::material::action::calendar_today::regular())
                                                    .size(34.0)
                                                    .color(fission::op::Color { r: 0, g: 0, b: 0, a: 255 })
                                            )
                                                .width(86.0)
                                                .height(80.0)
                                                .bg(fission::op::Color { r: 238, g: 238, b: 238, a: 255 })
                                                .border_radius(16.0)
                                                .padding([26.0, 26.0, 23.0, 23.0])),
                                            Widget::from(Text::new("Reserve").size(15.0).color(fission::op::Color { r: 10, g: 10, b: 10, a: 255 }).weight(600)),
                                        ],
                                        ..Default::default()
                                    })),
                                ],
                                ..Default::default()
                            })
                        ],
                        ..Default::default()
                    })
                    .width(core::f32::INFINITY) // CRITICAL: Stretch across full screen!
                    .bg(fission::op::Color { r: 255, g: 255, b: 255, a: 255 }) // True white bottom sheet
                    .border_radius(24.0)
                    .padding([24.0, 24.0, 24.0, 48.0])
                    .shadow(fission::op::BoxShadow {
                        color: fission::op::Color { r: 0, g: 0, b: 0, a: 30 },
                        blur_radius: 20.0,
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
                
            Widget::from(Container::new(ZStack {
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
            .width(core::f32::INFINITY)
            .height(core::f32::INFINITY))
        }
    }
