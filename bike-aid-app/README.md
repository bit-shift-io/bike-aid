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
Install dependencies
```bash
sudo pacman -S jdk11-openjdk clang lld llvm
```

Set env variables in ~/.bashrc
```bash
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_SDK_ROOT=$ANDROID_HOME
export PATH=$PATH:$ANDROID_HOME/tools
export PATH=$PATH:$ANDROID_HOME/platform-tools
```

Run ```sudo adb devices```


## wireless adb
Enable wireless debugging on android
Click it and get the ip, port and pair code.
```sudo adb pair ip:port```
insert paircode.
```sudo adb connect ip:port```

## run android
```bash
x doctor
x devices
x build
x build --device adb:<id>
x run --device adb:<id>
```



## Links

#### BLE

https://github.com/MnlPhlp/blec


#### Android
https://github.com/slint-ui/slint/tree/3aafce2c52fa1e14971e4f2455bdc556daeb079d/internal/backends/android-activity

https://github.com/slint-ui/slint/blob/master/examples/todo/rust/lib.rs

https://git.sr.ht/~tmpod/eigen/tree/main/item/src

https://github.com/search?q=repo%3Aslint-ui%2Fslint%20android&type=code

https://github.com/slint-ui/slint/blob/ed7eda02860ce9d0256268d034913aeece8e5f81/api/rs/slint/android.rs#L93

https://crates.io/crates/android-activity

https://docs.rs/android-manifest/latest/android_manifest/struct.UsesPermission.html

#### JNI

https://docs.rs/jni/latest/jni/

https://github.com/astonbitecode/j4rs

#### Build.rs

https://github.com/slint-ui/slint/blob/e0f3fd4168fa6ad5ed17a50910111ed72d1ae95e/internal/backends/android-activity/build.rs#L15
