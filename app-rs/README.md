# Bike Aid (Rust + Flutter)

A high-performance, multiplatform Bluetooth application built with **Flutter** for the UI and **Rust** for the core logic.

> **Architecture Note**: This project uses **Rust** for all core application logic (UUIDs, parsing, state management) and **Dart/Flutter** strictly for the GUI and device transport. This separation allows for easily swapping the GUI framework in the future while preserving the business logic.

## Project Structure

### 🛠️ Core Logic (Rust)
The "brain" of the app lives in the `rust/` directory.
- **protocol.rs**: **Entry Point.** Contains the scooter protocol implementation.
  - `ScooterState`: The central data structure representing the bike (speed, battery, light status, etc.).
  - `parse_characteristic_data()`: Converts raw bytes from the bike into the `ScooterState`.
  - `create_command_bytes()`: Generates the raw bytes to send to the bike for actions like Power, Alarm, or Horn.
- **`src/api/`**: Contains the modules exposed to Flutter.

### 📱 Frontend (Flutter)
The UI and Bluetooth transport are in the `lib/` directory.
- **main.dart**: **GUI Entry Point.**
  - **Bluetooth Manager**: Handles scanning for "BScooter", auto-connecting, and characteristic subscriptions.
  - **Swipe Navigation**: Implements the fullscreen `PageView` (Dashboard <-> Connectivity).
  - **Control Panel**: The button-based interface that triggers Rust commands.
- **`lib/src/rust/`**: **Generated Bridge.** Contains the Dart bindings that allow calling Rust functions seamlessly.

## Prerequisites

- **Flutter SDK**
- **Rust** (Stable)
- **Android NDK** (for Android builds)

## Setup & Running

### 1. Fix PATH & Install Tools (Once)
```fish
set -U fish_user_paths $HOME/.cargo/bin $fish_user_paths
cargo install flutter_rust_bridge_codegen --version 2.11.1
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

### 2. Generate Bridge
Run this to link the Rust code (even if logic is in Dart):
```bash
flutter_rust_bridge_codegen generate
```

### 3. Run the App
```bash
flutter run
```

## Features
- **Native Bluetooth**: Fast and reliable scanning using `flutter_blue_plus`.
- **Permission Handling**: Dynamic requests for Location and Bluetooth.
- **Dark Theme**: Modern Material 3 dark UI.

## Troubleshooting

- **Missing Imports**: If `main.dart` shows errors after generation, uncomment the bridge imports at the top of the file.
- **Cargo Expand**: If generation fails with "cargo expand returned empty output", run `cargo build` inside the `rust/` folder once.
