#!/bin/sh
set -eu

REPO="agent-clis/gripe"
INSTALL_DIR="${GRIPE_INSTALL_DIR:-/usr/local/bin}"

detect_platform() {
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Darwin) os="darwin" ;;
    Linux)  os="linux" ;;
    *)
      echo "Unsupported OS: $os" >&2
      exit 1
      ;;
  esac

  case "$arch" in
    x86_64|amd64)  arch="amd64" ;;
    arm64|aarch64) arch="arm64" ;;
    *)
      echo "Unsupported architecture: $arch" >&2
      exit 1
      ;;
  esac

  echo "${os}-${arch}"
}

main() {
  platform="$(detect_platform)"
  artifact="gripe-${platform}"
  url="https://github.com/${REPO}/releases/latest/download/${artifact}.tar.gz"

  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT

  echo "Downloading gripe for ${platform}..."
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$url" -o "$tmp/gripe.tar.gz"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$tmp/gripe.tar.gz" "$url"
  else
    echo "Error: curl or wget is required" >&2
    exit 1
  fi

  tar xzf "$tmp/gripe.tar.gz" -C "$tmp"
  mv "$tmp/${artifact}" "$tmp/gripe"
  chmod +x "$tmp/gripe"

  mkdir -p "$INSTALL_DIR"
  if [ -w "$INSTALL_DIR" ]; then
    mv "$tmp/gripe" "$INSTALL_DIR/gripe"
  else
    echo "Writing to ${INSTALL_DIR} requires sudo."
    sudo mv "$tmp/gripe" "$INSTALL_DIR/gripe"
  fi

  echo "gripe installed to ${INSTALL_DIR}/gripe"
}

main
