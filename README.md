# Fission UI Examples

This repository contains example applications built with [Fission](https://github.com/worka-ai/fission), a production-ready, cross-platform, GPU-accelerated UI framework for Rust. 

Fission uses a Flutter-inspired widget architecture with deterministic state management and Vello rendering. These examples demonstrate how to build rich, declarative UI applications that run across desktop (macOS, Windows, Linux), mobile (iOS, Android), and the web.

## Examples Included

This workspace includes multiple crates demonstrating different patterns and capabilities of the Fission framework.

### 📱 TikTok Clone (`crates/tiktok-fission`)

A mobile-first video feed application that showcases:
- **Complex Gestures & Swiping**: Handling touch and drag interactions.
- **Media Playback**: Integrating rich media widgets.
- **Local State Management**: Using `#[local_state]` for component-level data.
- **Redux-like Architecture**: Using the `#[fission_reducer]` macro for explicit state updates (e.g., `SwipeNext`, `ToggleLike`).
- **ZStack Compositing**: Layering UI elements like full-screen video with a bottom navigation bar overlay.

### 🚗 Uber Clone Suite (`crates/uber`)

A comprehensive ride-sharing platform that demonstrates how a single Rust crate can power multiple distinct applications while sharing a common domain model.

It defines three separate binaries:
- `rider`: The passenger application for booking rides and tracking trips.
- `driver`: The driver application for accepting requests and tracking earnings.
- `admin`: The back-office dashboard for monitoring users, drivers, and trips.

**Key Concepts Demonstrated:**
- **Global State Management**: Utilizing `fission::prelude::GlobalState` to manage application state and navigation.
- **Code Sharing**: Sharing rich domain models (`Trip`, `GeoAddress`, `FareEstimate`, etc.) across multiple entry points.
- **Routing**: Implementing a robust navigation stack and conditional screen routing based on the shared `AppState`.

## Prerequisites

To build and run these examples, you'll need the Fission CLI installed:

```bash
# Install the Fission CLI toolchain
cargo install cargo-fission
```

## Running the Examples

Fission provides its own CLI for a seamless development lifecycle. You can run any of the applications using `fission run`.

### Running the TikTok Clone

```bash
cd crates/tiktok-fission
fission run --target macos # Or ios, linux, windows, web
```

### Running the Uber Suite

Since the Uber example contains multiple binaries, you can run them individually:

```bash
cd crates/uber

# Run the Rider app
fission run --bin rider --target macos

# Run the Driver app
fission run --bin driver --target macos

# Run the Admin dashboard
fission run --bin admin --target macos
```

## Learn More about Fission

Fission makes building modern apps in Rust simple, providing:
- Struct-based widget composition in pure Rust using `#[fission_component]`.
- Built-in widget catalog and design system support (Material Design 3, Fluent 2, Cupertino).
- Out-of-the-box support for 3D scenes, Charts, and platform capability APIs (camera, biometrics, NFC).

Check out the [Fission GitHub Repository](https://github.com/worka-ai/fission) to learn more.
