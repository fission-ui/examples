use fission::core::ui::TextContent;
use fission::prelude::*;

#[fission_component]
#[derive(Clone)]
pub struct ProfileStat {
    pub count: String,
    pub label_key: &'static str,
}

impl From<ProfileStat> for Widget {
    fn from(stat: ProfileStat) -> Self {
        let (_ctx, view) = fission::build::current::<crate::state::TikTokState>();
        let tokens = &view.env().theme.tokens;

        fission::core::ui::Column {
            children: vec![
                fission::core::ui::Text::new(stat.count)
                    .color(tokens.colors.text_primary)
                    .weight(tokens.typography.font_weight_bold)
                    .into(),
                fission::core::ui::Text::new(TextContent::Key(stat.label_key.into()))
                    .color(tokens.colors.text_secondary)
                    .into(),
            ],
            ..Default::default()
        }
        .into()
    }
}
