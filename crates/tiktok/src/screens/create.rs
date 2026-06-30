use crate::state::TikTokState;
use crate::style::{black_alpha, rgba, white_alpha};
use fission::core::ui::TextContent;
use fission::op::{AlignItems, JustifyContent};
use fission::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
enum CreateMode {
    SixtySeconds,
    FifteenSeconds,
    Photo,
    Template,
}

impl Default for CreateMode {
    fn default() -> Self {
        Self::SixtySeconds
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct CreateLocalState {
    mode: CreateMode,
    recording: bool,
}

impl fission::core::GlobalState for CreateLocalState {}

#[fission_reducer(SetCreateMode)]
fn set_create_mode(local: &mut CreateLocalState, mode: CreateMode) {
    local.mode = mode;
}

#[fission_reducer(ToggleRecording)]
fn toggle_recording(local: &mut CreateLocalState) {
    local.recording = !local.recording;
}

#[fission_component]
#[derive(Clone, Default)]
pub struct CreateScreen {
    #[local_state(default = CreateLocalState::default())]
    local: CreateLocalState,
}

impl From<CreateScreen> for Widget {
    fn from(screen: CreateScreen) -> Self {
        let (ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;
        let local_handle = screen.local();
        let local = local_handle.get();
        let record_action = ctx.bind_local(
            ToggleRecording,
            local_handle.clone(),
            reduce!(toggle_recording),
        );

        let mut mode_widgets = Vec::new();
        for mode in [
            CreateMode::SixtySeconds,
            CreateMode::FifteenSeconds,
            CreateMode::Photo,
            CreateMode::Template,
        ] {
            let label_key = match mode {
                CreateMode::SixtySeconds => "create.mode_60",
                CreateMode::FifteenSeconds => "create.mode_15",
                CreateMode::Photo => "create.mode_photo",
                CreateMode::Template => "create.mode_template",
            };
            let active = local.mode == mode;
            let action = ctx.bind_local(
                SetCreateMode(mode.clone()),
                local_handle.clone(),
                reduce!(set_create_mode),
            );
            mode_widgets.push(
                CaptureModeChip {
                    label_key,
                    active,
                    on_tap: action,
                }
                .into(),
            );
        }

        Container::new(fission::widgets::ZStack {
            children: vec![
                Container::default().bg(tokens.colors.background).into(),
                fission::widgets::Positioned {
                    top: Some(54.0),
                    left: Some(16.0),
                    right: Some(16.0),
                    child: Some(
                        fission::core::ui::Row {
                            children: vec![
                                fission::core::ui::Text::new(TextContent::Key(
                                    "create.title".into(),
                                ))
                                .size(20.0)
                                .weight(tokens.typography.font_weight_bold)
                                .color(tokens.colors.text_primary)
                                .flex_grow(1.0)
                                .into(),
                                fission::core::ui::Text::new(TextContent::Key(
                                    "create.drafts".into(),
                                ))
                                .size(13.0)
                                .color(tokens.colors.text_secondary)
                                .into(),
                            ],
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            ..Default::default()
                        }
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    top: Some(92.0),
                    left: Some(14.0),
                    right: Some(76.0),
                    bottom: Some(168.0),
                    child: Some(
                        CapturePreview {
                            recording: local.recording,
                        }
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    top: Some(112.0),
                    right: Some(14.0),
                    child: Some(
                        fission::core::ui::Column {
                            children: vec![
                                CaptureTool {
                                    icon_svg:
                                        fission::icons::material::image::flip_camera_ios::round(),
                                    label_key: "create.flip",
                                }
                                .into(),
                                CaptureTool {
                                    icon_svg: fission::icons::material::av::speed::round(),
                                    label_key: "create.speed",
                                }
                                .into(),
                                CaptureTool {
                                    icon_svg: fission::icons::material::image::timer::round(),
                                    label_key: "create.timer",
                                }
                                .into(),
                                CaptureTool {
                                    icon_svg: fission::icons::material::image::auto_awesome::round(
                                    ),
                                    label_key: "create.effects",
                                }
                                .into(),
                                CaptureTool {
                                    icon_svg: fission::icons::material::image::music_note::round(),
                                    label_key: "create.sound",
                                }
                                .into(),
                            ],
                            gap: Some(18.0),
                            align_items: AlignItems::Center,
                            ..Default::default()
                        }
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    bottom: Some(102.0),
                    left: Some(0.0),
                    right: Some(0.0),
                    child: Some(
                        fission::core::ui::Column {
                            children: vec![
                                fission::core::ui::Row {
                                    children: mode_widgets,
                                    gap: Some(8.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                }
                                .into(),
                                RecordButton {
                                    recording: local.recording,
                                    on_tap: record_action,
                                }
                                .into(),
                            ],
                            gap: Some(16.0),
                            align_items: AlignItems::Center,
                            ..Default::default()
                        }
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        })
        .bg(tokens.colors.background)
        .flex_grow(1.0)
        .into()
    }
}

#[fission_component]
#[derive(Clone)]
struct CapturePreview {
    recording: bool,
}

impl From<CapturePreview> for Widget {
    fn from(preview: CapturePreview) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;
        let recording_label = if preview.recording { "REC" } else { "READY" };

        Container::new(fission::widgets::ZStack {
            children: vec![
                Container::default()
                    .bg(rgba(28, 30, 36, 255))
                    .border_radius(22.0)
                    .into(),
                fission::widgets::Positioned {
                    top: Some(16.0),
                    left: Some(16.0),
                    child: Some(
                        Container::new(
                            fission::core::ui::Text::new(recording_label)
                                .size(12.0)
                                .weight(tokens.typography.font_weight_bold)
                                .color(tokens.colors.text_primary),
                        )
                        .bg(if preview.recording {
                            tokens.colors.primary
                        } else {
                            black_alpha(90)
                        })
                        .border_radius(12.0)
                        .padding([10.0, 10.0, 5.0, 5.0])
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
                fission::widgets::Positioned {
                    bottom: Some(18.0),
                    left: Some(18.0),
                    right: Some(18.0),
                    child: Some(
                        fission::core::ui::Column {
                            children: vec![
                                fission::core::ui::Text::new(TextContent::Key(
                                    "create.preview_title".into(),
                                ))
                                .size(18.0)
                                .weight(tokens.typography.font_weight_bold)
                                .color(tokens.colors.text_primary)
                                .into(),
                                fission::core::ui::Text::new(TextContent::Key(
                                    "create.preview_hint".into(),
                                ))
                                .size(13.0)
                                .color(tokens.colors.text_secondary)
                                .max_lines(2)
                                .into(),
                            ],
                            gap: Some(5.0),
                            ..Default::default()
                        }
                        .into(),
                    ),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        })
        .border(white_alpha(28), 1.0)
        .border_radius(22.0)
        .into()
    }
}

#[fission_component]
#[derive(Clone)]
struct CaptureTool {
    icon_svg: &'static str,
    label_key: &'static str,
}

impl From<CaptureTool> for Widget {
    fn from(tool: CaptureTool) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;

        fission::core::ui::Column {
            children: vec![
                Container::new(crate::widgets::AppIcon {
                    svg: tool.icon_svg,
                    size: 21.0,
                    color: tokens.colors.text_primary,
                })
                .width(42.0)
                .height(42.0)
                .border_radius(21.0)
                .bg(white_alpha(24))
                .into(),
                fission::core::ui::Text::new(TextContent::Key(tool.label_key.into()))
                    .size(10.0)
                    .color(tokens.colors.text_secondary)
                    .into(),
            ],
            align_items: AlignItems::Center,
            gap: Some(4.0),
            ..Default::default()
        }
        .into()
    }
}

#[fission_component]
#[derive(Clone)]
struct CaptureModeChip {
    label_key: &'static str,
    active: bool,
    on_tap: ActionEnvelope,
}

impl From<CaptureModeChip> for Widget {
    fn from(chip: CaptureModeChip) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;
        let hit_id = WidgetId::explicit(&format!("create.mode.{}.hit", chip.label_key));
        let bg = if chip.active {
            white_alpha(42)
        } else {
            black_alpha(0)
        };

        let body = Container::new(
            fission::core::ui::Text::new(TextContent::Key(chip.label_key.into()))
                .size(12.0)
                .weight(if chip.active {
                    tokens.typography.font_weight_bold
                } else {
                    tokens.typography.font_weight_medium
                })
                .color(tokens.colors.text_primary),
        )
        .bg(bg)
        .border_radius(14.0)
        .padding([10.0, 10.0, 6.0, 6.0]);

        let tappable = fission::core::ui::widgets::GestureDetector {
            id: Some(hit_id),
            on_tap: Some(chip.on_tap),
            child: body.into(),
            ..Default::default()
        };

        tappable.into()
    }
}

#[fission_component]
#[derive(Clone)]
struct RecordButton {
    recording: bool,
    on_tap: ActionEnvelope,
}

impl From<RecordButton> for Widget {
    fn from(button: RecordButton) -> Self {
        let (_ctx, view) = fission::build::current::<TikTokState>();
        let tokens = &view.env().theme.tokens;
        let hit_id = WidgetId::explicit("create.record.button.hit");
        let pulse_track = fission::motion::MotionTrack::composite(
            fission::motion::MotionPropertyId::Scale,
            fission::motion::MotionStartValue::Explicit(fission::motion::scalar(1.0)),
            fission::motion::scalar(1.16),
        )
        .transition(
            fission::motion::MotionTransition::tween(820, fission::motion::MotionEasing::EaseInOut)
                .repeat(true)
                .frame_interval_ms(Some(33)),
        );

        let pulse = fission::motion::presence(
            "create.record.pulse",
            button.recording,
            vec![pulse_track],
            Container::default()
                .width(96.0)
                .height(96.0)
                .border_radius(48.0)
                .bg(tokens.colors.primary),
        );

        let core_size = if button.recording { 42.0 } else { 62.0 };
        let body = fission::widgets::ZStack {
            children: vec![
                pulse,
                Container::default()
                    .width(80.0)
                    .height(80.0)
                    .border_radius(40.0)
                    .border(tokens.colors.text_primary, 5.0)
                    .into(),
                Container::default()
                    .width(core_size)
                    .height(core_size)
                    .border_radius(if button.recording { 12.0 } else { 31.0 })
                    .bg(tokens.colors.primary)
                    .into(),
            ],
            ..Default::default()
        };

        let tappable = fission::core::ui::widgets::GestureDetector {
            id: Some(hit_id),
            on_tap: Some(button.on_tap),
            child: Container::new(body).width(104.0).height(104.0).into(),
            ..Default::default()
        };

        tappable.into()
    }
}
