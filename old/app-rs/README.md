# Bike-Aid-App

A Rust application that's using [Slint](https://slint.rs) for the user interface. Using bluetooth low energy (BLE) and desktop and mobile app.

## VSCode plugins
Slint  
rust-analyzer  
CodeLLDB  

## Setup Android
Install sdk & ndk in the ~/Android/ directory  

Install dependencies  
```bash
sudo pacman -S jdk17-openjdk 
# clang lld llvm - use android ndk tools instead
```

Install android tools
```bash
sudo pacman -S android-tools android-udev
```

Set env variables in ~/.bash_profile 
```bash
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/28.0.12674087/
export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME
export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin

# for skia bindings
export ANDROID_NDK=$ANDROID_NDK_HOME
export CC_aarch64_linux_android=aarch64-linux-android26-clang
export CXX_aarch64_linux_android=aarch64-linux-android26-clang++
export AR_aarch64_linux_android=llvm-ar
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android26-clang
```

Install rust tools
```bash
rustup update
rustup target add aarch64-linux-android
cargo install cargo-ndk
```

Run and test
```sudo adb devices```


## Android Gradle Build
``` bash
cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs/  build --features=android
cd android
./gradlew build
./gradlew installDebug
```

working as an apk... need libs?
```bash
cargo apk run --target aarch64-linux-android --lib
```

## Android xbuild -old?
```bash
x doctor
x devices
x build
x build --device adb:<id>
x run --device adb:<id>
```

## Android cargo-apk - untested
```bash
rustup target add aarch64-linux-android
cargo install cargo-apk
cargo apk build
cargo apk run
```


## Build external packages
```
<cd to Projects folder>
git clone https://github.com/deviceplug/jni-utils-rs.git
cd jni-utils-rs
cargo build --features=build-java-support
jar files located in /target/debug/java/libs

cd ..
git clone https://github.com/deviceplug/btleplug.git
cd btleplug/src/droidplug/java
edit build.gradle, comment and replace:
dependencies {
    implementation files('/home/bronson/Projects/jni-utils-rs/target/debug/java/libs/jni-utils-0.1.1-SNAPSHOT.jar')
}
./gradlew build 
btleplug/src/droidplug/java/build/outputs/aar/
```
an optional idea could be to merge the 2 android folders and build?


## Getting rust working
Run ```cargo build``` in the rust project root directory  
Then ```cargo run```
