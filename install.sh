#!/usr/bin/env sh
set -eu

REPO="${ODK_REPO:-HanZawNyein/odoo_developer_kit}"
INSTALL_DIR="${ODK_INSTALL_DIR:-$HOME/.local/bin}"
BIN_NAME="odk"
PACKAGE_NAME="odoo-developer-kit"

detect_target() {
  os_name="$(uname -s 2>/dev/null || echo unknown)"
  arch_name="$(uname -m 2>/dev/null || echo unknown)"

  case "$os_name:$arch_name" in
    Darwin:arm64|Darwin:aarch64) echo "aarch64-apple-darwin" ;;
    Darwin:x86_64) echo "x86_64-apple-darwin" ;;
    Linux:x86_64|Linux:amd64) echo "x86_64-unknown-linux-gnu" ;;
    Linux:aarch64|Linux:arm64) echo "aarch64-unknown-linux-gnu" ;;
    MINGW*:x86_64|MSYS*:x86_64|CYGWIN*:x86_64) echo "x86_64-pc-windows-msvc" ;;
    *)
      echo "Unsupported platform: $os_name $arch_name" >&2
      exit 1
      ;;
  esac
}

download_url() {
  target="$1"
  case "$target" in
    *windows*|*msvc*) echo "https://github.com/${REPO}/releases/latest/download/${PACKAGE_NAME}-${target}.zip" ;;
    *) echo "https://github.com/${REPO}/releases/latest/download/${PACKAGE_NAME}-${target}.tar.xz" ;;
  esac
}

ensure_path_instruction() {
  case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
      echo
      echo "Add ODK to your PATH:"
      echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
      ;;
  esac
}

main() {
  target="$(detect_target)"
  url="$(download_url "$target")"
  tmp_dir="$(mktemp -d)"
  archive="$tmp_dir/odk-release"
  extract_dir="$tmp_dir/extract"

  mkdir -p "$INSTALL_DIR" "$extract_dir"

  echo "Detected target: $target"
  echo "Downloading: $url"
  curl -LsSf "$url" -o "$archive"

  case "$url" in
    *.zip)
      if ! command -v unzip >/dev/null 2>&1; then
        echo "unzip is required to install the Windows archive." >&2
        exit 1
      fi
      unzip -q "$archive" -d "$extract_dir"
      ;;
    *.tar.xz)
      tar -xJf "$archive" -C "$extract_dir"
      ;;
    *)
      echo "Unsupported archive format: $url" >&2
      exit 1
      ;;
  esac

  binary_path="$(find "$extract_dir" -type f \( -name "$BIN_NAME" -o -name "$BIN_NAME.exe" \) | head -n 1)"
  if [ -z "$binary_path" ]; then
    echo "Downloaded archive did not contain the $BIN_NAME binary." >&2
    exit 1
  fi

  cp "$binary_path" "$INSTALL_DIR/$BIN_NAME"
  chmod +x "$INSTALL_DIR/$BIN_NAME"

  if "$INSTALL_DIR/$BIN_NAME" --help >/dev/null 2>&1; then
    echo "Odoo Developer Kit installed successfully."
    echo "Run: odk doctor"
    ensure_path_instruction
  else
    echo "Installation verification failed." >&2
    exit 1
  fi
}

main "$@"
