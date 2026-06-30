use crate::data::VideoData;
use crate::style::{tiktok_cyan, tiktok_red};
use fission::op::AlignItems;
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct ActionSidebar {
    pub video: VideoData,
    pub is_liked: bool,
    pub is_saved: bool,
    pub is_following: bool,
    pub display_likes: u64,
    pub display_saves: u64,
    pub on_like: ActionEnvelope,
    pub on_comments: ActionEnvelope,
    pub on_save: ActionEnvelope,
    pub on_follow: ActionEnvelope,
}

impl Default for ActionSidebar {
    fn default() -> Self {
        panic!("ActionSidebar must be initialized with data");
    }
}

impl From<ActionSidebar> for Widget {
    fn from(sidebar: ActionSidebar) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;

        Container::new(fission::core::ui::Column {
            align_items: AlignItems::Center,
            children: vec![
                crate::widgets::CreatorAvatar {
                    handle: sidebar.video.user.handle.clone(),
                    display_name: sidebar.video.user.display_name.clone(),
                    avatar_color: sidebar.video.user.avatar_color.clone(),
                    is_following: sidebar.is_following,
                    on_follow: sidebar.on_follow,
                }
                .into(),
                Container::default().height(20.0).into(),
                crate::widgets::FeedActionButton {
                    id: format!("{}.like", sidebar.video.id),
                    icon_svg: fission::icons::material::action::favorite::round(),
                    count: crate::state::format_count(sidebar.display_likes),
                    active: sidebar.is_liked,
                    accent: tiktok_red(),
                    on_tap: Some(sidebar.on_like),
                }
                .into(),
                crate::widgets::FeedActionButton {
                    id: format!("{}.comments", sidebar.video.id),
                    icon_svg: fission::icons::material::communication::comment::round(),
                    count: crate::state::format_count(sidebar.video.comments),
                    active: false,
                    accent: tiktok_cyan(),
                    on_tap: Some(sidebar.on_comments),
                }
                .into(),
                crate::widgets::FeedActionButton {
                    id: format!("{}.save", sidebar.video.id),
                    icon_svg: fission::icons::material::action::bookmark::round(),
                    count: crate::state::format_count(sidebar.display_saves),
                    active: sidebar.is_saved,
                    accent: tiktok_cyan(),
                    on_tap: Some(sidebar.on_save),
                }
                .into(),
                crate::widgets::FeedActionButton {
                    id: format!("{}.share", sidebar.video.id),
                    icon_svg: fission::icons::material::content::send::round(),
                    count: crate::state::format_count(sidebar.video.shares),
                    active: false,
                    accent: tokens.colors.text_primary,
                    on_tap: None,
                }
                .into(),
                crate::widgets::RecordDisc {
                    video_id: sidebar.video.id.clone(),
                    avatar_color: sidebar.video.user.avatar_color.clone(),
                }
                .into(),
            ],
            gap: Some(16.0),
            ..Default::default()
        })
        .width(60.0)
        .into()
    }
}
