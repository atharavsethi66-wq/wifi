# WiFi Login iOS Application

A mobile application built using Tauri v2 that wraps your existing Rust `wifi_connect` library. It lets users enter credentials and connects them to the college Wi-Fi network.

---

## Project Structure

```text
wifi_app/
├── src-tauri/
│   ├── Cargo.toml         # References wifi_connect crate
│   ├── build.rs           # Tauri build script
│   ├── tauri.conf.json    # Configuration with global Tauri enabled
│   ├── src/
│   │   ├── lib.rs         # Shared Rust entry and commands
│   │   └── main.rs        # Desktop entry point
│   ├── permissions/
│   │   └── default.toml   # Allow frontend to call login_wifi command
│   └── capabilities/
│       └── default.json   # Grant permissions to main window
│
├── src/
│   ├── index.html         # Minimal UI (username, password, Login, status)
│   ├── style.css          # Simple, clean form styling
│   └── main.js            # Calls tauri invoke("login_wifi")
│
├── wifi_connect/          # Core Rust networking logic crate
└── README.md
```

---

## Technical Specifications

1. **Frontend:** Clean HTML/CSS/JS (vanilla). Accesses Tauri commands through `window.__TAURI__.core.invoke`.
2. **Backend Command (`login_wifi`):** Defined in `src-tauri/src/lib.rs`. It bridges inputs from the UI to the local `wifi_connect` Rust library.
3. **Rust Core:** `wifi_connect` executes the POST requests via `reqwest` and parses output via `quick-xml`.

---

## Building the iOS Application & Generating IPA

Since building an iOS IPA requires Xcode and the iOS SDK, compiling the project must be executed on a macOS environment.

### 1. Prerequisite Setup on macOS

Before building the application, install the iOS targets and developer CLI tool:
```bash
# Add the iOS compilation targets
rustup target add aarch64-apple-ios x86_64-apple-ios

# Install the Tauri CLI tool globally or run it via npx
cargo install tauri-cli
```

### 2. Initialize iOS Project
To generate the Xcode project template structure:
```bash
cargo tauri ios init
```
This command initializes the native Xcode files under `src-tauri/gen/apple/`.

### 3. Open in Xcode for Code Signing
To run on a physical iPhone or upload to the App Store, you must configure code signing:
1. Open the project in Xcode:
   ```bash
   open src-tauri/gen/apple/wifi-app_iOS/wifi-app_iOS.xcodeproj
   ```
2. Navigate to **Signing & Capabilities**.
3. Select your **Apple Developer Team** and select **Automatically manage signing**.

### 4. Build and Compile the `.ipa`
To compile the package:
```bash
cargo tauri ios build
```

This compiles your Rust library for Apple architectures, embeds your web assets, compiles the Swift wrapper, and exports a final build package.
