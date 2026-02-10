#!/usr/bin/env zsh
set -eu

TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "x86_64-linux-android")
OUTPUT_DIR="../../Android/app/src/main/jniLibs/"
BIN_DIR=$(dirname "$0")
ANDROID_DIR="${BIN_DIR}/../../android"
ANDROID_DIR=$(realpath "$ANDROID_DIR")
GRADLE_PROPERTIES_FILE="${ANDROID_DIR}/gradle.properties"
read_properties() {
  file="$1"
  key="$2"
  value=$(grep "^$key=" "$file")
  if [ -z "$value" ]; then
    echo "$key not found in $file"
    exit 1
  fi
  value=$(echo "$value" | awk -F= '{print $2}' | tr -d ' ')
  echo "$value"
}

ANDROID_BUILD_NDK_VERSION=$(read_properties "$GRADLE_PROPERTIES_FILE" "ANDROID_BUILD_NDK_VERSION")
ANDROID_SDK_ROOT="$HOME/Library/Android/sdk"
export ANDROID_NDK_HOME="$ANDROID_SDK_ROOT/ndk/$ANDROID_BUILD_NDK_VERSION"
if [ ! -d "$ANDROID_NDK_HOME" ]; then
  echo "Directory '$ANDROID_NDK_HOME' does not exist or is not a directory."
  exit 1
fi
for target in "${TARGETS[@]}"; do
    echo "Building for target: $target"
    cargo ndk --platform 21 --target "$target" --output-dir "$OUTPUT_DIR" --no-strip build
done