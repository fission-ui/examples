use crate::state::TikTokState;
use crate::style::{black_alpha, true_black};
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone, Default)]
pub struct BottomNav {}

impl From<BottomNav> for Widget {
    fn from(_: BottomNav) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let state = view.state();
        let tokens = &view.env().theme.tokens;
        let current_path = &state.current_path;

        let bar_bg = if current_path == "/" {
            black_alpha(218)
        } else {
            true_black()
        };

        Container::new(fission::core::ui::Row {
            children: vec![
                crate::widgets::NavItem {
                    target_path: "/",
                    icon_svg: fission::icons::material::action::home::round(),
                    label_key: "nav.home",
                }
                .into(),
                crate::widgets::NavItem {
                    target_path: "/discover",
                    icon_svg: fission::icons::material::action::explore::round(),
                    label_key: "nav.discover",
                }
                .into(),
                crate::widgets::CreateNavButton::default().into(),
                crate::widgets::NavItem {
                    target_path: "/inbox",
                    icon_svg: fission::icons::material::content::inbox::round(),
                    label_key: "nav.inbox",
                }
                .into(),
                crate::widgets::NavItem {
                    target_path: "/profile",
                    icon_svg: fission::icons::material::action::account_circle::round(),
                    label_key: "nav.profile",
                }
                .into(),
            ],
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .bg(bar_bg)
        .border(tokens.colors.border, 0.0)
        .height(80.0)
        .into()
    }
}
