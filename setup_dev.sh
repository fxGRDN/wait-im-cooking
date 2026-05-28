#!/usr/bin/env bash
set -euo pipefail

command_exists() {
  command -v "$1" >/dev/null 2>&1
}

skip() {
  echo "Skipping: $*"
}

if command_exists pacman; then
  sudo pacman -Syu --needed rustup
else
  skip "pacman not found (rustup install)."
fi

if command_exists rustup; then
  rustup default stable

  targets=(
    aarch64-linux-android
    armv7-linux-androideabi
    i686-linux-android
    x86_64-linux-android
    aarch64-apple-ios
    x86_64-apple-ios
  )

  installed_targets="$(rustup target list --installed)"
  for target in "${targets[@]}"; do
    if echo "$installed_targets" | grep -qx "$target"; then
      skip "rustup target $target already installed."
    else
      rustup target add "$target"
    fi
  done
else
  skip "rustup not found (target setup)."
fi

if command_exists yay; then
  yay -S --needed android-ndk sdkmanager android-sdk-cmdline-tools-latest
else
  skip "yay not found (Android packages install)."
fi

if command_exists sdkmanager; then
  # Accept licenses
  yes | sdkmanager --licenses

  # List available NDK versions
  sdkmanager --list | grep ndk || true

  # Install specific version (Tauri 2 works best with r26 / 26.x)
  ndk_version="26.3.11579264"
  if [[ -n "${ANDROID_SDK_ROOT:-}" && -d "${ANDROID_SDK_ROOT}/ndk/${ndk_version}" ]]; then
    skip "NDK ${ndk_version} already installed in ANDROID_SDK_ROOT."
  elif [[ -n "${ANDROID_HOME:-}" && -d "${ANDROID_HOME}/ndk/${ndk_version}" ]]; then
    skip "NDK ${ndk_version} already installed in ANDROID_HOME."
  else
    sdkmanager "ndk;${ndk_version}"
  fi
else
  skip "sdkmanager not found (NDK setup)."
fi
