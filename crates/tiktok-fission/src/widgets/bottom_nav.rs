use fission::prelude::*;
use fission::op::{AlignItems, JustifyContent};
use crate::state::TikTokState;

#[fission_component]
#[derive(Clone, Default)]
pub struct BottomNav {}

impl From<BottomNav> for Widget {
    fn from(_: BottomNav) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let state = view.state();
        let tokens = &view.env().theme.tokens;
        let current_path = &state.current_path;

        let bar_bg = if current_path == "/" {
            fission::op::Color { r: 15, g: 15, b: 15, a: 230 }
        } else {
            fission::op::Color { r: 0, g: 0, b: 0, a: 255 }
        };

        Container::new(
            fission::core::ui::Row {
                children: vec![
                    nav_item(&ctx, current_path, "/", "🏠", "Home", tokens).into(),
                    nav_item(&ctx, current_path, "/discover", "🔍", "Discover", tokens).into(),
                    create_btn(&ctx, tokens).into(),
                    nav_item(&ctx, current_path, "/inbox", "📬", "Inbox", tokens).into(),
                    nav_item(&ctx, current_path, "/profile", "👤", "Me", tokens).into(),
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
}

fn nav_item(
    ctx: &BuildCtxHandle<TikTokState>,
    current_path: &str,
    target_path: &str,
    icon: &str,
    label: &str,
    tokens: &fission::theme::Tokens,
) -> Widget {
    let active = current_path == target_path;
    let color = if active { tokens.colors.text_primary } else { tokens.colors.text_secondary };
    
    let action = ctx.bind(
        crate::state::SetCurrentPath(target_path.to_string()),
        reduce!(crate::state::reduce_set_current_path)
    );

    let inner = Container::new(
        fission::core::ui::Column {
            children: vec![
                fission::core::ui::Text::new(icon.to_string()).into(),
                Container::default().height(2.0).into(),
                fission::core::ui::Text::new(label.to_string()).into(),
            ],
            align_items: AlignItems::Center,
            ..Default::default()
        }
    )
    .width(64.0)
    .padding([12.0, 0.0, 12.0, 0.0]);

    fission::core::ui::widgets::GestureDetector {
        on_tap: Some(action),
        child: inner.into(),
        ..Default::default()
    }.into()
}

fn create_btn(ctx: &BuildCtxHandle<TikTokState>, tokens: &fission::theme::Tokens) -> Widget {
    let action = ctx.bind(
        crate::state::SetCurrentPath("/create".to_string()),
        reduce!(crate::state::reduce_set_current_path)
    );

    let inner = Container::new(
        fission::widgets::ZStack {
            children: vec![
                fission::widgets::Positioned {
                    top: Some(0.0), left: Some(0.0),
                    child: Some(
                        Container::default().bg(tokens.colors.secondary).width(40.0).height(30.0).border_radius(7.0).into()
                    ),
                    ..Default::default()
                }.into(),
                fission::widgets::Positioned {
                    top: Some(0.0), right: Some(0.0),
                    child: Some(
                        Container::default().bg(tokens.colors.primary).width(40.0).height(30.0).border_radius(7.0).into()
                    ),
                    ..Default::default()
                }.into(),
                fission::widgets::Positioned {
                    top: Some(0.0), left: Some(4.0),
                    child: Some(
                        Container::new(fission::core::ui::Text::new("+"))
                            .bg(tokens.colors.text_primary)
                            .width(40.0)
                            .height(30.0)
                            .border_radius(7.0)
                            .into()
                    ),
                    ..Default::default()
                }.into(),
            ],
            ..Default::default()
        }
    )
    .width(48.0)
    .padding([8.0, 0.0, 8.0, 0.0]);

    fission::core::ui::widgets::GestureDetector {
        on_tap: Some(action),
        child: inner.into(),
        ..Default::default()
    }.into()
}
