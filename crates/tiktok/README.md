# tiktok

A Fission example that recreates a TikTok-style mobile app surface with a
full-screen video feed, layered action chrome, animated controls, creator
profile, discovery, inbox, and create flows.

## Highlights

- DSP-backed custom design system generated from `assets/theme/dsp.json`.
- Global app state for route, likes, saves, follows, trends, notifications, and
  locale/theme sync.
- `#[local_state]` for feed playback/swipe/comment state and create-screen
  recording controls.
- Motion API usage for feed chrome, comments presence, action hover/press,
  record pulse, trend/card entrance, sound pill entrance, and record-disc
  rotation.
- Bundled demo content in `src/assets/data.json` and local videos in
  `src/assets/videos`.

## Targets

Configured in `fission.toml`:

- `android`
- `ios`
- `linux`
- `macos`
- `windows`

## Commands

- `cargo check -p tiktok`
- `fission doctor --project-dir .`
- `fission devices --project-dir .`
- `fission run --project-dir .`
- `fission run --target ios --project-dir .`
- `fission run --target android --project-dir .`
- `fission build --target <target> --project-dir . --release`
