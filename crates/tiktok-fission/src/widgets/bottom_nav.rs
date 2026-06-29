use fission::prelude::*;
use fission::op::{AlignItems, JustifyContent};
use fission::core::StateField;
use crate::state::{Tab, TikTokState};
use crate::ui_helpers::*;
use crate::app;

/// Build the TikTok-style bottom navigation bar
pub fn build_bottom_nav(
    ctx: &BuildCtxHandle<()>,
    state_handle: &StateField<TikTokState>,
) -> Widget {
    let state = state_handle.get();
    let active_tab = state.active_tab;

    // Transparent on home (over video), solid dark on other tabs
    let bar_bg = if active_tab == Tab::Home {
        rgba(15, 15, 15, 230)
    } else {
        rgba(0, 0, 0, 255)
    };

    Container::new(
        Row {
            children: vec![
                nav_item(ctx, state_handle, Tab::Home,     "🏠", "Home",     active_tab == Tab::Home),
                nav_item(ctx, state_handle, Tab::Discover, "🔍", "Discover", active_tab == Tab::Discover),
                create_btn(ctx, state_handle),
                nav_item(ctx, state_handle, Tab::Inbox,    "📬", "Inbox",    active_tab == Tab::Inbox),
                nav_item(ctx, state_handle, Tab::Profile,  "👤", "Me",       active_tab == Tab::Profile),
            ],
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..Default::default()
        }
    )
    .bg(bar_bg)
    .height(80.0)
    .into()
}

fn nav_item(
    ctx: &BuildCtxHandle<()>,
    state_handle: &StateField<TikTokState>,
    tab: Tab,
    icon: &str,
    label: &str,
    active: bool,
) -> Widget {
    let color = if active { WHITE } else { INACTIVE_GREY };
    let action = app::switch_tab_action(ctx, state_handle, tab);

    on_tap(
        Container::new(
            Column {
                children: vec![
                    text(icon, if active { 24.0 } else { 22.0 }, color).into(),
                    Container::default().height(2.0).into(),
                    text(label, 10.0, color).into(),
                ],
                align_items: AlignItems::Center,
                ..Default::default()
            }
        )
        .width(64.0)
        .padding([12.0, 0.0, 12.0, 0.0]),
        action
    )
}

/// TikTok's signature create button:
/// Cyan rect (offset left) + Red rect (offset right) + White rect on top with "+"
fn create_btn(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>) -> Widget {
    let action = app::switch_tab_action(ctx, state_handle, Tab::Create);

    on_tap(
        Container::new(
            ZStack {
                children: vec![
                    // Cyan left layer
                    pos_tl(0.0, 0.0,
                        Container::default().bg(TIKTOK_CYAN).width(40.0).height(30.0).border_radius(7.0)
                    ).into(),
                    // Red right layer
                    pos_tr(0.0, 0.0,
                        Container::default().bg(TIKTOK_RED).width(40.0).height(30.0).border_radius(7.0)
                    ).into(),
                    // White centre layer with "+"
                    pos_tl(4.0, 0.0,
                        Container::new(text_bold("+", 22.0, BLACK))
                            .bg(WHITE)
                            .width(40.0)
                            .height(30.0)
                            .border_radius(7.0)
                    ).into(),
                ],
                ..Default::default()
            }
        )
        .width(48.0)
        .padding([8.0, 0.0, 8.0, 0.0]),
        action
    )
}
