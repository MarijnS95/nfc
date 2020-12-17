set -e

cargo ndk --target aarch64-linux-android --platform 29 b
adb push target/aarch64-linux-android/debug/nfc data/local/
adb shell chmod +x /data/local/nfc
adb shell /data/local/nfc
