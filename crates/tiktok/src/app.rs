use fission::core::ui::{TextContent, Widget};
use fission::i18n::{I18nRegistry, Locale, TranslationBundle};
use fission::prelude::*;
use fission::theme::DesignSystem;
use fission::widgets::{Positioned, Route, Router, ZStack};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::design_system::TikTokDesignSystem;
use crate::screens::{CreateScreen, DiscoverScreen, FeedScreen, InboxScreen, ProfileScreen};
use crate::state::TikTokState;

// ─── Root App Component ──────────────────────────────────────────────────────

#[derive(Clone, Default)]
pub struct TikTokApp;

impl From<TikTokApp> for Widget {
    fn from(_app: TikTokApp) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();

        let router = Router::<TikTokState> {
            current_path: view.state().current_path.clone(),
            routes: vec![
                Route {
                    path: "/".into(),
                    builder: Arc::new(|_, _, _| FeedScreen::default().into()),
                },
                Route {
                    path: "/discover".into(),
                    builder: Arc::new(|_, _, _| DiscoverScreen::default().into()),
                },
                Route {
                    path: "/create".into(),
                    builder: Arc::new(|_, _, _| CreateScreen::default().into()),
                },
                Route {
                    path: "/inbox".into(),
                    builder: Arc::new(|_, _, _| InboxScreen::default().into()),
                },
                Route {
                    path: "/profile".into(),
                    builder: Arc::new(|_, _, _| ProfileScreen::default().into()),
                },
            ],
            not_found: Some(Arc::new(|_, _, _| {
                fission::core::ui::Text::new(TextContent::Key("route.not_found".into())).into()
            })),
        };

        ZStack {
            children: vec![
                router.into(),
                Positioned {
                    bottom: Some(0.0),
                    left: Some(0.0),
                    right: Some(0.0),
                    child: Some(crate::widgets::BottomNav::default().into()),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        }
        .into()
    }
}

pub fn sync_env(state: &TikTokState, env: &mut Env) {
    env.locale = state.locale.clone();
    env.i18n = i18n_registry().clone();
    env.theme = TikTokDesignSystem::theme(state.theme_mode);
    env.window.title = WindowTitle::plain(
        i18n_registry()
            .get(&state.locale, "app.title")
            .unwrap_or("TikTok"),
    );
}

fn i18n_registry() -> &'static I18nRegistry {
    static REGISTRY: OnceLock<I18nRegistry> = OnceLock::new();

    REGISTRY.get_or_init(|| {
        let mut registry = I18nRegistry::new();
        registry.add_bundle(load_bundle("en-US", include_str!("i18n/en-US.json")));
        registry.add_bundle(load_bundle("es-ES", include_str!("i18n/es-ES.json")));
        registry
    })
}

fn load_bundle(locale: &str, raw_json: &str) -> TranslationBundle {
    let messages: HashMap<String, String> =
        serde_json::from_str(raw_json).expect("translation bundle JSON must be valid");

    TranslationBundle {
        locale: Locale::from(locale),
        messages,
    }
}
