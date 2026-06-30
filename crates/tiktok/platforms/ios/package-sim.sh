#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)
PROJECT_DIR=$(cd -- "$SCRIPT_DIR/../.." && pwd)
TARGET="${IOS_SIM_TARGET:-aarch64-apple-ios-sim}"
PROFILE="${IOS_SIM_PROFILE:-debug}"
PACKAGE_NAME="tiktok"
BUNDLE_ID="${IOS_BUNDLE_ID:-com.example.tiktok}"
DISPLAY_NAME="${IOS_DISPLAY_NAME:-TiktokFission}"
EXECUTABLE_NAME="${IOS_EXECUTABLE_NAME:-tiktok}"
BUNDLE_NAME="${IOS_BUNDLE_NAME:-$DISPLAY_NAME.app}"
BUILD_DIR="$SCRIPT_DIR/build/$PROFILE"
BUNDLE_DIR="$BUILD_DIR/$BUNDLE_NAME"

BUILD_ARGS=(build --manifest-path "$PROJECT_DIR/Cargo.toml" --target "$TARGET" --package "$PACKAGE_NAME" --bin "$PACKAGE_NAME")
ARTIFACT_DIR=debug
if [[ "$PROFILE" == "release" ]]; then
  BUILD_ARGS+=(--release)
  ARTIFACT_DIR=release
fi

cargo "${BUILD_ARGS[@]}"
TARGET_DIR=$(python3 - <<'PY' "$PROJECT_DIR/Cargo.toml"
import json
import subprocess
import sys

manifest = sys.argv[1]
metadata = json.loads(
    subprocess.check_output(
        ["cargo", "metadata", "--manifest-path", manifest, "--format-version", "1", "--no-deps"]
    )
)
print(metadata["target_directory"])
PY
)

rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR"
# For iOS, use the binary executable
if [[ -f "$TARGET_DIR/$TARGET/$ARTIFACT_DIR/$PACKAGE_NAME" ]]; then
  cp "$TARGET_DIR/$TARGET/$ARTIFACT_DIR/$PACKAGE_NAME" "$BUNDLE_DIR/$EXECUTABLE_NAME"
else
  echo "Error: Could not find iOS binary at $TARGET_DIR/$TARGET/$ARTIFACT_DIR/$PACKAGE_NAME" >&2
  ls -la "$TARGET_DIR/$TARGET/$ARTIFACT_DIR/" | grep -E tiktok >&2
  exit 1
fi
chmod +x "$BUNDLE_DIR/$EXECUTABLE_NAME"
python3 - <<'PY' "$SCRIPT_DIR/Info.plist" "$BUNDLE_DIR/Info.plist" "$BUNDLE_ID" "$DISPLAY_NAME" "$EXECUTABLE_NAME"
import plistlib
import sys

source, dest, bundle_id, display_name, executable_name = sys.argv[1:]
with open(source, "rb") as handle:
    plist = plistlib.load(handle)
plist["CFBundleIdentifier"] = bundle_id
plist["CFBundleDisplayName"] = display_name
plist["CFBundleName"] = display_name
plist["CFBundleExecutable"] = executable_name
with open(dest, "wb") as handle:
    plistlib.dump(plist, handle, sort_keys=False)
PY
shopt -s nullglob
PLATFORM_APP_ICONS=("$SCRIPT_DIR"/AppIcon.*)
if (( ${#PLATFORM_APP_ICONS[@]} == 0 )); then
  cp "$PROJECT_DIR/assets/app-icon.png" "$BUNDLE_DIR/AppIcon.png"
else
  app_icon="${PLATFORM_APP_ICONS[0]}"
  cp "$app_icon" "$BUNDLE_DIR/$(basename "$app_icon")"
fi
shopt -u nullglob
shopt -s nullglob
SPLASH_IMAGES=("$SCRIPT_DIR"/SplashImage.*)
if (( ${#SPLASH_IMAGES[@]} == 0 )); then
  cp "$PROJECT_DIR/assets/app-icon.png" "$BUNDLE_DIR/SplashImage.png"
else
  for splash_image in "${SPLASH_IMAGES[@]}"; do
    cp "$splash_image" "$BUNDLE_DIR/"
  done
fi
shopt -u nullglob
if [[ -f "$SCRIPT_DIR/LaunchScreen.storyboard" ]]; then
  IBTOOL=$(xcrun --find ibtool 2>/dev/null || true)
  if [[ -z "$IBTOOL" ]]; then
    printf 'ibtool not found. Install Xcode command line tools to compile the iOS launch screen storyboard.\n' >&2
    printf 'Skipping LaunchScreen.storyboard compilation.\n' >&2
  else
    "$IBTOOL" \
      --errors \
      --warnings \
      --notices \
      --target-device iphone \
      --target-device ipad \
      --minimum-deployment-target 13.0 \
      --output-format human-readable-text \
      --compile "$BUNDLE_DIR/LaunchScreen.storyboardc" \
      "$SCRIPT_DIR/LaunchScreen.storyboard" 2>&1 > /dev/null || {
        printf 'Warning: LaunchScreen.storyboard compilation failed. App will launch without custom launch screen.\n' >&2
      }
  fi
fi
printf 'APPL????' > "$BUNDLE_DIR/PkgInfo"
printf '%s\n' "$BUNDLE_DIR"
