# Rust Universal Starter

A Rust template for web, desktop and terminal applications. Leptos 0.8 renders the same interface in the browser and the Tauri 2 desktop window. Ratatui renders a terminal interface with the same state and actions from `app-core`.

## Run it

Install [Rust via rustup](https://rustup.rs/). The repository pins Rust 1.100.0 and declares the WebAssembly target, rustfmt and Clippy in `rust-toolchain.toml`. Run these setup commands from the repository root for web and desktop development.

```sh
rustup show
cargo install trunk --version 0.21.14 --locked
cargo install tauri-cli --version 2.12.6 --locked
```

The terminal app only needs Rust and an interactive terminal. For desktop, install the [Tauri 2 system prerequisites](https://v2.tauri.app/start/prerequisites/) for your host. On Debian and Ubuntu, these include WebKitGTK 4.1 and the native build tools. macOS requires Xcode Command Line Tools. Windows requires the MSVC C++ build tools and WebView2.

| Interface | Development command |
| --- | --- |
| Web | `trunk serve` |
| Desktop | `cargo tauri dev` |
| Terminal | `cargo run -p universal-tui` |

Open [http://localhost:1420](http://localhost:1420) for web development. Tauri starts Trunk on the same port, so stop the standalone web server before running desktop development.

## Android and iOS

Install the [Tauri mobile prerequisites](https://v2.tauri.app/start/prerequisites/#configure-for-mobile-targets) for the platform you want to build. With the pinned Tauri CLI, Android needs SDK Platform 37.0, Android Build-Tools, and the NDK. iOS needs macOS and full Xcode. Install the corresponding Rust targets as part of that setup.

Initialize each platform once from the repository root, after setting your project's Tauri bundle identifier and package names. Then run the app on an available emulator, simulator, or device.

| Platform | Initialize | Develop | Build |
| --- | --- | --- | --- |
| Android | `cargo tauri android init` | `make android` | `make build-android` |
| iOS | `cargo tauri ios init` | `make ios` | `make build-ios` |

Pass extra Tauri CLI options through `ANDROID_ARGS` or `IOS_ARGS`. For an Android debug APK for an x86-64 emulator, run `make build-android ANDROID_ARGS="--debug --apk --target x86_64"`. For physical devices, see the [Tauri development server notes](https://v2.tauri.app/develop/#development-server). Mobile development uses the same Trunk server on port 1420, so stop any standalone web or desktop development session first.

Configure the platform's [signing credentials](https://v2.tauri.app/distribute/#signing) before distributing an Android or iOS build. The mobile targets run independently of the default `make build`.

## First run

- [ ] Open Home and increment the counter.
- [ ] Open Activity and confirm that it shows the same count. Visit Settings and return to Home to confirm the count survives navigation.
- [ ] Press `1`, `2` or `3` to open Home, Activity or Settings. Press `I` to increment. In Web and Desktop, use Tab to put keyboard focus inside the app first.
- [ ] Repeat the interaction in a narrow window and in a small terminal. Use `Q`, Esc or Ctrl+C to exit the terminal app.

Each running interface starts its own session.

## Repository layout

| Path | Purpose |
| --- | --- |
| `crates/app-core/` | Framework-independent `AppState`, `Screen`, `Action` and `update` |
| `src/`, `styles/` | Leptos shell, navigation and panels shared by Web and Desktop |
| `src-tauri/` | Tauri window, capabilities, bundling configuration and icons |
| `tui/` | Ratatui rendering and Crossterm input |
| `index.html`, `Trunk.toml` | WebAssembly entry page and asset build |

## Customize a new project

On GitHub, use **Use this template → Create a new repository**, then clone the new repository. GitHub copies the files as they are, so rename the starter values in your checkout.

| Setting | Example | Where to edit |
| --- | --- | --- |
| Repository name | `my-ai-app` | GitHub’s new repository form |
| Rust packages | `my-ai-app-ui`, `my-ai-app`, `my-ai-app-tui` | `Cargo.toml`, `src-tauri/Cargo.toml`, `tui/Cargo.toml`. Update the Tauri `[lib].name` to `my_ai_app_lib` and its reference in `src-tauri/src/main.rs`. |
| Application title | My AI App | `index.html`, `src/components/shell.rs`, `tui/src/ui.rs`, `productName` and `app.windows[0].title` in `src-tauri/tauri.conf.json` |
| Bundle identifier | `com.example.myaiapp` | `identifier` in `src-tauri/tauri.conf.json` |
| Application icon | Your square source image | Run `cargo tauri icon path/to/icon.png`. The default output is `src-tauri/icons/`. |
| Theme | Your colors and spacing | CSS variables in `styles/app.css` and the Ratatui accent color in `tui/src/ui.rs` |

After renaming, run `cargo check -p app-core` to refresh the workspace package names in `Cargo.lock`. Commit the lockfile and update the terminal command in your README to use your new package name.

## Build and verify

With GNU Make installed, run `make help` for the available targets. Use `make web`, `make desktop`, or `make tui` for development, `make check` for the local quality checks, and `make build` for all three release outputs. The aggregate check and build targets use the desktop prerequisites.

```sh
cargo fmt --all -- --check
cargo test -p app-core
cargo check -p app-core --target wasm32-unknown-unknown
cargo check -p universal-tui
trunk build --release
cargo tauri build
```

Deploy the web bundle from `dist/` to a static host. For a project hosted under a path such as `/my-ai-app/`, pass `--public-url /my-ai-app/` to the web build. Tauri’s development URL matches the Trunk port, and Tauri’s production build consumes `../dist` relative to `src-tauri/`.

Linux uses a Debian package by default. To build another Linux format, pass `--bundles` and install the packaging prerequisites for that format.

CI runs formatting, Clippy, core tests, the WebAssembly build and the terminal build check. For desktop it builds unsigned Debian, macOS DMG and Windows NSIS packages and uploads them with the web bundle on the workflow run.

## Upstream and license

| Upstream | Reference |
| --- | --- |
| [Tauri Create App Leptos preset](https://github.com/tauri-apps/create-tauri-app/tree/create-tauri-app-v4.8.0/templates/template-leptos) | Generator 4.8.0, used as the application base |
| [Ratatui Simple](https://github.com/ratatui/templates/tree/5903368da4cfce85144f8efc7070f9c175a8f203/simple-generated) | Reference for the synchronous terminal loop |
| [Leptos Start Trunk](https://github.com/leptos-rs/start-trunk/tree/9bbd0600a4bf2c7c6e5303816c1373f647434c4d) | Reference for the static web build |

The starter is [MIT-licensed](LICENSE). The [upstream Tauri notice](TAURI-LICENSE-MIT) covers generated material carried into this repository.
