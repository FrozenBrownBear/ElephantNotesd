#!/usr/bin/env bash
# Auto‑installer multiplateforme (Linux, macOS Intel/Apple Silicon, Windows via Git Bash)

set -e

echo "▶ Détection de la plateforme…"
UNAME_S="$(uname -s)"
UNAME_M="$(uname -m)"

case "${UNAME_S}-${UNAME_M}" in
  Darwin-arm64)
    PLATFORM="macOS‑AppleSilicon"
    TARGET="aarch64-apple-darwin"
    ;;
  Darwin-x86_64)
    PLATFORM="macOS‑Intel"
    TARGET="x86_64-apple-darwin"
    ;;
  Linux-*)
    PLATFORM="Linux"
    TARGET="x86_64-unknown-linux-gnu"
    ;;
  MINGW*|MSYS*|CYGWIN*-*)
    PLATFORM="Windows"
    TARGET="x86_64-pc-windows-gnu"
    ;;
  *)
    echo "⚠️  Plateforme inconnue (${UNAME_S}-${UNAME_M}), compilation locale"
    TARGET=""
    ;;
esac
echo "   → $PLATFORM (target : $TARGET)"

# Installe rustup si nécessaire
if ! command -v rustup >/dev/null 2>&1; then
  echo "▶ Installation du toolchain Rust…"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

# Ajoute la cible détectée
if [ -n "$TARGET" ]; then
  rustup target add "$TARGET" || true
fi

# Compile en mode release
echo "▶ Compilation (release)…"
cargo build --release

echo "✅ Compilation terminée ! Binaire dans target/release/"