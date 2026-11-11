#!/usr/bin/env bash
#
# install-linux.sh — first-time install of the Codex AppImage on any x86-64 Linux.
#
# Run:  curl -fsSL https://raw.githubusercontent.com/electricm0nk/codex/develop/scripts/install-linux.sh | bash
#
# It downloads the newest release's AppImage, checks its sha256, and saves it under a FIXED name
# (no version in it), because the in-app updater rewrites that same file on every update. It also
# adds a `codex` command and an app-menu entry. After this, updates happen inside the app.
set -euo pipefail

REPO="electricm0nk/codex"
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}"
APP_DIR="$DATA_DIR/codex"
APPIMAGE="$APP_DIR/Codex.AppImage"
BIN_LINK="$HOME/.local/bin/codex"
DESKTOP_FILE="$DATA_DIR/applications/codex.desktop"
ICON="$APP_DIR/codex.png"

die() { echo "error: $*" >&2; exit 1; }

[ "$(uname -m)" = "x86_64" ] || die "only x86-64 Linux builds exist (this machine is $(uname -m))."
for tool in curl sha256sum; do command -v "$tool" >/dev/null || die "'$tool' is required."; done

# shortcut: a reinstall over an existing file would leave the updater's installed-state record
# pointing at a different sha256; refuse until there is a real reinstall/repair path.
[ -e "$APPIMAGE" ] && die "Codex is already installed at $APPIMAGE. Use the app's update panel instead."

# Newest release first (all releases are pre-releases today, so /releases/latest would 404).
release_json=$(curl -fsSL "https://api.github.com/repos/$REPO/releases?per_page=1")
asset_url() { printf '%s\n' "$release_json" | grep -o '"browser_download_url": *"[^"]*'"$1"'"' | head -1 | sed 's/.*"\(https[^"]*\)"/\1/'; }
appimage_url=$(asset_url '_amd64\.AppImage')
sums_url=$(asset_url 'checksums\.sha256')
[ -n "$appimage_url" ] && [ -n "$sums_url" ] || die "could not find the AppImage in the newest release."
appimage_name=${appimage_url##*/}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
echo "Downloading $appimage_name ..."
curl -fL --progress-bar -o "$tmp/$appimage_name" "$appimage_url"
curl -fsSL -o "$tmp/checksums.sha256" "$sums_url"
expected=$(awk -v f="./$appimage_name" '$2 == f || $2 == substr(f, 3) { print $1 }' "$tmp/checksums.sha256")
[ -n "$expected" ] || die "checksums.sha256 has no line for $appimage_name."
actual=$(sha256sum "$tmp/$appimage_name" | awk '{ print $1 }')
[ "$expected" = "$actual" ] || die "sha256 mismatch for $appimage_name (expected $expected, got $actual)."
echo "Checksum OK."

mkdir -p "$APP_DIR" "$(dirname "$BIN_LINK")" "$(dirname "$DESKTOP_FILE")"
install -m 755 "$tmp/$appimage_name" "$APPIMAGE"
ln -sfn "$APPIMAGE" "$BIN_LINK"
curl -fsSL -o "$ICON" "https://raw.githubusercontent.com/$REPO/develop/apps/desktop/src-tauri/icons/128x128.png" \
  || echo "warning: could not download the icon; the menu entry will have none." >&2
cat > "$DESKTOP_FILE" <<DESKTOP
[Desktop Entry]
Type=Application
Name=Codex
Comment=Tabletop RPG character builder and sheet printer
Exec=$APPIMAGE
Icon=$ICON
Categories=Game;Utility;
Terminal=false
DESKTOP

echo
echo "Installed $appimage_name as $APPIMAGE"
echo "Start it from your app menu, or run: codex"
case ":$PATH:" in *":$HOME/.local/bin:"*) ;; *) echo "note: $HOME/.local/bin is not on your PATH, so 'codex' will not work until you add it." ;; esac
if ! { ldconfig -p 2>/dev/null || true; } | grep -q 'libfuse\.so\.2'; then
  echo "note: FUSE 2 was not found. AppImages need it. Install it with one of:"
  echo "  Arch:            sudo pacman -S fuse2"
  echo "  Ubuntu 24.04+:   sudo apt install libfuse2t64"
  echo "  Fedora:          sudo dnf install fuse-libs"
fi
echo "If the window opens blank, start it once with: WEBKIT_DISABLE_DMABUF_RENDERER=1 codex"
