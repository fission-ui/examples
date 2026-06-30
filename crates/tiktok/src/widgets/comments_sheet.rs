use crate::data::VideoData;
use crate::style::{black_alpha, white_alpha};
use fission::core::ui::TextContent;
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct CommentsSheet {
    pub video: VideoData,
    pub on_close: ActionEnvelope,
}

impl From<CommentsSheet> for Widget {
    fn from(sheet: CommentsSheet) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;
        let title = format!(
            "{} {}",
            crate::state::format_count(sheet.video.comments),
            view.env()
                .i18n
                .get(&view.env().locale, "feed.comments")
                .unwrap_or("comments")
        );

        let close = fission::core::ui::widgets::GestureDetector {
            on_tap: Some(sheet.on_close),
            child: Container::new(
                fission::core::ui::Text::new("x")
                    .size(16.0)
                    .weight(tokens.typography.font_weight_bold)
                    .color(tokens.colors.text_primary),
            )
            .width(32.0)
            .height(32.0)
            .border_radius(16.0)
            .bg(white_alpha(28))
            .into(),
            ..Default::default()
        };

        let comments = fission::core::ui::Column {
            children: vec![
                CommentPreview {
                    author: "Mina".into(),
                    body: "The footwork on count five is too clean.".into(),
                    avatar_color: "#FE2C55".into(),
                }
                .into(),
                CommentPreview {
                    author: "Ari".into(),
                    body: "Saved this for tomorrow's practice.".into(),
                    avatar_color: "#25F4EE".into(),
                }
                .into(),
                CommentPreview {
                    author: "Cam".into(),
                    body: format!("Need the full tutorial for {}.", sheet.video.category),
                    avatar_color: "#F59E0B".into(),
                }
                .into(),
            ],
            gap: Some(14.0),
            ..Default::default()
        };

        Container::new(fission::core::ui::Column {
            children: vec![
                fission::core::ui::Row {
                    children: vec![
                        fission::core::ui::Text::new(title)
                            .size(16.0)
                            .weight(tokens.typography.font_weight_bold)
                            .color(tokens.colors.text_primary)
                            .flex_grow(1.0)
                            .into(),
                        close.into(),
                    ],
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                }
                .into(),
                comments.into(),
                Container::new(fission::core::ui::Text::new(TextContent::Key(
                    "feed.comment_prompt".into(),
                )))
                .bg(black_alpha(70))
                .border(white_alpha(22), 1.0)
                .border_radius(20.0)
                .padding([16.0, 16.0, 10.0, 10.0])
                .into(),
            ],
            gap: Some(16.0),
            ..Default::default()
        })
        .bg(black_alpha(232))
        .border(white_alpha(32), 1.0)
        .border_radius(18.0)
        .padding([18.0, 18.0, 18.0, 18.0])
        .into()
    }
}

#[fission_component]
#[derive(Clone)]
struct CommentPreview {
    author: String,
    body: String,
    avatar_color: String,
}

impl From<CommentPreview> for Widget {
    fn from(comment: CommentPreview) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;

        fission::core::ui::Row {
            children: vec![
                crate::widgets::CompactAvatar {
                    id: format!("comment.{}", comment.author),
                    label: comment.author.clone(),
                    avatar_color: comment.avatar_color,
                    size: 34.0,
                }
                .into(),
                fission::core::ui::Column {
                    children: vec![
                        fission::core::ui::Text::new(comment.author)
                            .size(12.0)
                            .weight(tokens.typography.font_weight_bold)
                            .color(tokens.colors.text_primary)
                            .into(),
                        fission::core::ui::Text::new(comment.body)
                            .size(13.0)
                            .color(tokens.colors.text_secondary)
                            .into(),
                    ],
                    gap: Some(2.0),
                    ..Default::default()
                }
                .into(),
            ],
            gap: Some(10.0),
            align_items: AlignItems::Start,
            ..Default::default()
        }
        .into()
    }
}
