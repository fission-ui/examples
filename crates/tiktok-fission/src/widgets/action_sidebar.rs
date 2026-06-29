use fission::prelude::*;
use fission::op::AlignItems;
use crate::data::VideoData;
use crate::state::format_count;

#[fission_component]
#[derive(Clone)]
pub struct ActionSidebar {
    pub video: VideoData,
    pub is_liked: bool,
    pub display_likes: u64,
    pub on_like: ActionEnvelope,
}

impl Default for ActionSidebar {
    fn default() -> Self {
        panic!("ActionSidebar must be initialized with data");
    }
}

impl From<ActionSidebar> for Widget {
    fn from(sidebar: ActionSidebar) -> Self {
        let (ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;

        let column = fission::core::ui::Column {
            align_items: AlignItems::Center,
            children: vec![
                build_avatar(&sidebar.video.user.name, tokens),
                Container::default().height(20.0).into(),
                action_btn(
                    if sidebar.is_liked { tokens.colors.primary } else { tokens.colors.text_primary },
                    sidebar.display_likes,
                    Some(sidebar.on_like),
                    tokens
                ),
                Container::default().height(20.0).into(),
                action_btn(tokens.colors.text_primary, sidebar.video.comments, None, tokens),
                Container::default().height(20.0).into(),
                action_btn(tokens.colors.text_secondary, sidebar.video.shares, None, tokens),
                Container::default().height(24.0).into(),
                build_disc(tokens),
                Container::default().height(8.0).into(),
            ],
            ..Default::default()
        };

        Container::new(column).width(68.0).into()
    }
}

fn build_avatar(_username: &str, tokens: &fission::theme::Tokens) -> Widget {
    Container::default()
        .bg(fission::op::Color { r: 80, g: 80, b: 80, a: 255 })
        .border_radius(27.0)
        .width(54.0)
        .height(54.0)
        .into()
}

fn action_btn(
    icon_color: fission::op::Color,
    count: u64,
    action: Option<ActionEnvelope>,
    tokens: &fission::theme::Tokens,
) -> Widget {
    let icon = Container::default()
        .bg(icon_color)
        .border_radius(20.0)
        .width(40.0)
        .height(40.0)
        .into();

    let inner = fission::core::ui::Column {
        align_items: AlignItems::Center,
        children: vec![
            icon,
            Container::default().height(4.0).into(),
            fission::core::ui::Text::new(format_count(count)).into(),
        ],
        ..Default::default()
    };

    match action {
        Some(a) => fission::core::ui::widgets::GestureDetector {
            on_tap: Some(a),
            child: inner.into(),
            ..Default::default()
        }.into(),
        None => inner.into(),
    }
}

fn build_disc(tokens: &fission::theme::Tokens) -> Widget {
    Container::default()
        .bg(fission::op::Color { r: 30, g: 30, b: 30, a: 255 })
        .border_radius(28.0)
        .width(56.0)
        .height(56.0)
        .into()
}
