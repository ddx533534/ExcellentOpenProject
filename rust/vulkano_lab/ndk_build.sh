#!/usr/bin/env zsh

set -eu

# shellcheck disable=SC2006
BIN_DIR=`dirname "$0"`
ANDROID_DIR="${BIN_DIR}/../Android"
ANDROID_DIR=`realpath "$ANDROID_DIR"`
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
ANDROID_BUILD_NDK_VERSION=25.1.8937393
ANDROID_SDK_ROOT="$HOME/Library/Android/sdk"
export ANDROID_NDK_HOME="$ANDROID_SDK_ROOT/ndk/$ANDROID_BUILD_NDK_VERSION"
if [ ! -d "$ANDROID_NDK_HOME" ]; then
  echo "Directory '$ANDROID_NDK_HOME' does not exist or is not a directory."
  exit 1
fi

cargo ndk --platform 21 --target armv7-linux-androideabi --output-dir ../Android/ --no-strip build
