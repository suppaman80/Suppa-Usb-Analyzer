#!/usr/bin/env bash
set -Eeuo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"

if [[ "${1:-}" != "--in-terminal" \
  && ! -t 1 \
  && ( -n "${DISPLAY:-}" || -n "${WAYLAND_DISPLAY:-}" ) \
  && -x "$(command -v konsole 2>/dev/null || true)" ]]; then
  exec konsole --hold -e bash "$SCRIPT_DIR/build-appimage.sh" --in-terminal
fi

cd "$SCRIPT_DIR"
source "$HOME/.cargo/env"

case "$(uname -m)" in
  x86_64|amd64) APPIMAGE_ARCH="x86_64" ;;
  *)
    printf 'Unsupported architecture: %s\n' "$(uname -m)" >&2
    exit 1
    ;;
esac

TOOL_CACHE="$HOME/.cache/tauri"
LINUXDEPLOY="$(find "$TOOL_CACHE" -maxdepth 1 -type f -name 'linuxdeploy*-x86_64.AppImage' -print -quit)"
APPIMAGE_PLUGIN="$TOOL_CACHE/linuxdeploy-plugin-appimage.AppImage"
if [[ -z "$LINUXDEPLOY" || ! -x "$LINUXDEPLOY" || ! -x "$APPIMAGE_PLUGIN" ]]; then
  printf 'linuxdeploy or AppImage plugin not found in %s\n' "$TOOL_CACHE" >&2
  exit 1
fi

VERSION="$(sed -n 's/^version = "\([^"]*\)".*/\1/p' Cargo.toml | head -n 1)"
TIMESTAMP="$(date '+%Y%m%d_%H%M%S')"
APPIMAGE_NAME="Suppa-Usb-Analyzer_${VERSION}_${TIMESTAMP}_${APPIMAGE_ARCH}.AppImage"
APPDIR="$SCRIPT_DIR/target/appimage/Suppa-Usb-Analyzer.AppDir_${TIMESTAMP}"
OUTPUT_DIR="$SCRIPT_DIR/artifacts/appimage"
OUTPUT_FILE="$OUTPUT_DIR/$APPIMAGE_NAME"
DESKTOP_FILE="$SCRIPT_DIR/suppa-usb-analyzer.desktop"
ICON_FILE="$SCRIPT_DIR/assets/suppa-usb-analyzer.png"
APP_EXECUTABLE="$SCRIPT_DIR/target/release/suppa-usb-analyzer"

mkdir -p "$OUTPUT_DIR"
cargo build --release
install -Dm644 "$DESKTOP_FILE" "$APPDIR/usr/share/applications/suppa-usb-analyzer.desktop"
install -Dm644 "$ICON_FILE" "$APPDIR/usr/share/icons/hicolor/512x512/apps/suppa-usb-analyzer.png"
install -Dm644 "$SCRIPT_DIR/LICENSE" "$APPDIR/usr/share/licenses/suppa-usb-analyzer/LICENSE"

PATH="$TOOL_CACHE:$PATH" \
LINUXDEPLOY_PLUGIN_DIR="$TOOL_CACHE" \
APPIMAGE_EXTRACT_AND_RUN=1 \
ARCH="$APPIMAGE_ARCH" \
LDAI_OUTPUT="$OUTPUT_FILE" \
"$LINUXDEPLOY" \
  --appdir "$APPDIR" \
  --executable "$APP_EXECUTABLE" \
  --desktop-file "$APPDIR/usr/share/applications/suppa-usb-analyzer.desktop" \
  --icon-file "$ICON_FILE" \
  --output appimage

chmod u+x "$OUTPUT_FILE"
printf '\nAppImage ready:\n%s\n' "$OUTPUT_FILE"
du -h -- "$OUTPUT_FILE"
