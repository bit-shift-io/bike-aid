# Bike-Aid-App

A Rust application that's using [Slint](https://slint.rs) for the user interface.

## About

Using bluetooth low energy (BLE) and desktop and mobile app.

## VSCode plugins
Slint  
rust-analyzer  
CodeLLDB  

## Getting rust working
Run ```cargo build``` in the rust project root directory  
Then ```cargo run```

## Setup Android
Install sdk & ndk in the ~/Android/ directory  
Note: may need to symlink clang??

Install dependencies  
```bash
sudo pacman -S jdk17-openjdk clang lld llvm
```

Install android tools
```bash
sudo pacman -S android-tools android-udev
```

Set env variables in ~/.bash_profile 
```bash
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/28.0.12674087/
export PATH=$PATH:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin

#export ANDROID_NDK_ROOT=$HOME/Android/Ndk
#export ANDROID_SDK_ROOT=$ANDROID_HOME

#export PATH=$PATH:$ANDROID_HOME/tools
#export PATH=$PATH:$ANDROID_HOME/platform-tools
```

Install rust tools
```bash
rustup update
rustup target add aarch64-linux-android
cargo install cargo-ndk
```

Run ```sudo adb devices```


## Android Gradle Build
``` bash
cargo ndk -t arm64-v8a -o app/src/main/jniLibs/  build
./gradlew build
./gradlew installDebug
```

working as an apk... need libs?
```bash
cargo apk run --target aarch64-linux-android --lib
```

## run android using xbuild -old?
```bash
x doctor
x devices
x build
x build --device adb:<id>
x run --device adb:<id>
```

## run android using cargo-apk - untested
```bash
rustup target add arm-linux-androideabi
cargo apk check
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


## Links

#### BLE

https://github.com/MnlPhlp/blec


#### Android
https://github.com/slint-ui/slint/tree/3aafce2c52fa1e14971e4f2455bdc556daeb079d/internal/backends/android-activity

https://github.com/slint-ui/slint/blob/master/examples/todo/rust/lib.rs

https://git.sr.ht/~tmpod/eigen/tree/main/item/src

https://github.com/search?q=repo%3Aslint-ui%2Fslint%20android&type=code


https://crates.io/crates/android-activity

https://docs.rs/android-manifest/latest/android_manifest/struct.UsesPermission.html

#### JNI

https://docs.rs/jni/latest/jni/

https://github.com/astonbitecode/j4rs

#### Build.rs

https://github.com/slint-ui/slint/blob/e0f3fd4168fa6ad5ed17a50910111ed72d1ae95e/internal/backends/android-activity/build.rs#L15
