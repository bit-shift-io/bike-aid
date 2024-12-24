.ONESHELL:
ANDROID_NATIVE_API_LEVEL = 25
NDK_VERSION ?= 25.1.8937393
NDK_HOME ?= /opt/android-sdk/ndk/$(NDK_VERSION)
SERVER ?= http://localhost:8000
ANDROID_OPTIONS ?=
# --features tauri
DESKTOP_OPTIONS ?=
# --features tauri
SERVER_OPTIONS ?=


init:
	export NDK_HOME=$(NDK_HOME)
	cargo tauri android init
	echo "Don't foget to set your jvm version in ~/.gradle/gradle.properties (/usr/lib/jvm/...)"
	cargo tauri icon icon.png

# Dev
run:
	SERVER=$(SERVER) cargo tauri dev $(DESKTOP_OPTIONS)

update:
	cargo update

doc:
	cargo doc --open

outdated-dependencies:
	cargo outdated

format:
	cargo fmt

check-format:
	cargo fmt --all -- --check
	cargo clippy

test:
	cargo test

# Desktop
desktop-build-debug:
	cargo tauri build $(DESKTOP_OPTIONS)

desktop-build:
	cargo tauri build $(DESKTOP_OPTIONS)

# Server
server-build:
	SERVER=$(SERVER) cargo build $(SERVER_OPTIONS)

server-run:
	SERVER=$(SERVER) cargo run $(SERVER_OPTIONS)

# Android
android-dev: .android-prepare
#	adb uninstall loikki.open_biking | true
	export NDK_HOME=$(NDK_HOME)
	export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig/:$PKG_CONFIG_PATH
	export PKG_CONFIG_SYSROOT_DIR=/
#	cargo tauri android dev $(ANDROID_OPTIONS)

.android-prepare:
	cp android-files/key.properties gen/android/
	cp android-files/build.gradle.kts gen/android/app/
	cp android-files/proguard-rules.pro gen/android/app
	cp -f -r android-files/jni-utils-rs/java/src/main/java/io gen/android/app/src/main/java/
	cp -f -r android-files/btleplug/src/droidplug/java/src/main/java/com gen/android/app/src/main/java/
	cp android-files/MainActivity.kt gen/android/app/src/main/java/loikki/open_biking/
	cp android-files/AndroidManifest.xml gen/android/app/src/main/

android-build: .android-prepare
	export NDK_HOME=$(NDK_HOME)
	cargo tauri android build $(ANDROID_OPTIONS)
	cp gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk open-biking.apk

android-build-debug: .android-prepare
	export NDK_HOME=$(NDK_HOME)
	cargo tauri android build $(ANDROID_OPTIONS) --debug
	cp gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk open-biking.apk

android-generate-key:
	keytool -genkey -v -keystore upload-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias upload

android-deploy:
	adb uninstall loikki.open_biking | true
	adb install open-biking.apk

android-logs:
	pid=`adb shell pidof loikki.open_biking`
	adb logcat --pid=$$pid
	echo $$pid


android-logs-clear:
	adb logcat -c

android-install-sdk:
	sudo sdkmanager --install "ndk;$(NDK_VERSION)"
	sudo sdkmanager --install "build-tools;33.0.0"
	sudo sdkmanager --install "platforms;android-$(ANDROID_NATIVE_API_LEVEL)"
	sudo sdkmanager --install "platform-tools"
	sudo sdkmanager --licenses

android-add-target:
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
