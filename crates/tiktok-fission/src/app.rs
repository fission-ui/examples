use fission::prelude::*;
use fission::core::StateField;
use crate::data;
use crate::state::{Tab, TikTokState};
use crate::screens;
use crate::widgets::bottom_nav;
use crate::ui_helpers::pos_bottom_bar;

// ─── Root App Component ──────────────────────────────────────────────────────

#[fission_component]
#[derive(Clone, Default)]
pub struct TikTokApp {
    #[local_state(default = TikTokState::default())]
    state: TikTokState,
}

// ─── Reducers ────────────────────────────────────────────────────────────────

#[fission_reducer(InitFeed)]
fn init_feed(state: &mut TikTokState) {
    if !state.is_loaded {
        let feed = data::load_feed();
        state.videos = feed.videos;
        state.is_loaded = true;
        state.is_playing = true;
        state.current_video_index = 0;
    }
}

#[fission_reducer(SwitchTab)]
pub fn switch_tab(state: &mut TikTokState, tab: Tab) {
    state.active_tab = tab;
}

#[fission_reducer(SwipeNext)]
pub fn swipe_next(state: &mut TikTokState) {
    if state.current_video_index < state.videos.len() - 1 {
        state.current_video_index += 1;
        state.is_playing = true;
    }
}

#[fission_reducer(SwipePrev)]
pub fn swipe_prev(state: &mut TikTokState) {
    if state.current_video_index > 0 {
        state.current_video_index -= 1;
        state.is_playing = true;
    }
}

#[fission_reducer(TogglePlayPause)]
pub fn toggle_play_pause(state: &mut TikTokState) {
    state.is_playing = !state.is_playing;
}

#[fission_reducer(ToggleLike)]
pub fn toggle_like(state: &mut TikTokState, video_id: String) {
    if state.liked_videos.contains(&video_id) {
        state.liked_videos.remove(&video_id);
    } else {
        state.liked_videos.insert(video_id);
    }
}

// ─── Action Builders ─────────────────────────────────────────────────────────

pub fn drag_start_action(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>) -> ActionEnvelope {
    ctx.bind_local(crate::state::DragStart, state_handle.clone(), crate::state::reduce_drag_start as fn(&mut TikTokState, crate::state::DragStart))
}

pub fn drag_update_action(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>) -> ActionEnvelope {
    ctx.bind_local(crate::state::DragUpdate, state_handle.clone(), crate::state::reduce_drag_update as fn(&mut TikTokState, crate::state::DragUpdate, &mut ReducerContext<TikTokState>))
}

pub fn drag_end_action(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>) -> ActionEnvelope {
    ctx.bind_local(crate::state::DragEnd, state_handle.clone(), crate::state::reduce_drag_end as fn(&mut TikTokState, crate::state::DragEnd))
}

pub fn swipe_next_action(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>) -> ActionEnvelope {
    ctx.bind_local(SwipeNext, state_handle.clone(), reduce!(swipe_next))
}

pub fn swipe_prev_action(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>) -> ActionEnvelope {
    ctx.bind_local(SwipePrev, state_handle.clone(), reduce!(swipe_prev))
}

pub fn toggle_play_pause_action(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>) -> ActionEnvelope {
    ctx.bind_local(TogglePlayPause, state_handle.clone(), reduce!(toggle_play_pause))
}

pub fn toggle_like_action(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>, video_id: String) -> ActionEnvelope {
    ctx.bind_local(ToggleLike(video_id), state_handle.clone(), reduce!(toggle_like))
}

pub fn switch_tab_action(ctx: &BuildCtxHandle<()>, state_handle: &StateField<TikTokState>, tab: Tab) -> ActionEnvelope {
    ctx.bind_local(SwitchTab(tab), state_handle.clone(), reduce!(switch_tab))
}

// ─── Component Implementation ───────────────────────────────────────────────────

impl From<TikTokApp> for Widget {
    fn from(app: TikTokApp) -> Self {
        let (ctx, _) = fission::build::current::<()>();
        let state_handle = app.state();

        let s = state_handle.get();

        // Build the active screen
        let screen: Widget = match s.active_tab {
            Tab::Home => screens::feed::build_feed(&ctx, &state_handle),
            Tab::Discover => screens::discover::build_discover(),
            Tab::Create => screens::create::build_create(),
            Tab::Inbox => screens::inbox::build_inbox(),
            Tab::Profile => screens::profile::build_profile(&s),
        };

        // Root layout: ZStack with screen content + bottom nav overlay
        ZStack {
            children: vec![
                // Full-screen content
                screen,
                // Bottom nav pinned to bottom edge
                pos_bottom_bar(0.0, bottom_nav::build_bottom_nav(&ctx, &state_handle)).into(),
            ],
            ..Default::default()
        }
        .into()
    }
}
