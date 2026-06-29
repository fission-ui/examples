use fission::core::ui::Widget;
use fission::prelude::*;
use fission::widgets::{Route, Router, ZStack, Positioned};
use fission::i18n::{Locale, TranslationBundle};
use std::collections::HashMap;
use std::sync::Arc;

use crate::state::TikTokState;
use crate::screens::{FeedScreen, DiscoverScreen, CreateScreen, InboxScreen, ProfileScreen};
use crate::widgets::BottomNav;

// ─── Root App Component ──────────────────────────────────────────────────────

#[derive(Clone, Default)]
pub struct TikTokApp;

impl From<TikTokApp> for Widget {
    fn from(_app: TikTokApp) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        
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
                fission::core::ui::Text::new("Page not found").into()
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
                }.into(),
            ],
            ..Default::default()
        }
        .into()
    }
}

pub fn create_env() -> Env {
    let mut env = Env::default();
    
    let mut en_messages = HashMap::new();
    en_messages.insert("nav.home".into(), "Home".into());
    en_messages.insert("nav.discover".into(), "Discover".into());
    en_messages.insert("nav.inbox".into(), "Inbox".into());
    en_messages.insert("nav.profile".into(), "Profile".into());
    
    let mut es_messages = HashMap::new();
    es_messages.insert("nav.home".into(), "Inicio".into());
    es_messages.insert("nav.discover".into(), "Descubrir".into());
    es_messages.insert("nav.inbox".into(), "Bandeja".into());
    es_messages.insert("nav.profile".into(), "Perfil".into());
    
    env.i18n.add_bundle(TranslationBundle {
        locale: Locale("en-US".into()),
        messages: en_messages,
    });
    
    env.i18n.add_bundle(TranslationBundle {
        locale: Locale("es-ES".into()),
        messages: es_messages,
    });
    
    env
}
