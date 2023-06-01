# 🆎 About This Template

This [`flutter-rust-app-template`](https://github.com/cunarist/flutter-rust-app-template) provides instant capabilities to developers who want to embrace the power of **Flutter** and **Rust** together. Simply duplicate the template repo and you're ready to go!

![preview](https://github.com/cunarist/flutter-rust-app-template/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

This template is primarily built using the [`flutter_rust_bridge`](https://github.com/fzyzcjy/flutter_rust_bridge) library. It also incorporates several popular packages and modifications into the default Flutter template, ensuring optimal development process. It has been designed with future scalability and performance in mind.

Extra features added to the default Flutter template are:

- Rust integration with the ability to use an arbitrary number of library crates
- MVVM pattern with easy viewmodel binding from Dart and viewmodel update from Rust
- Restarting Rust logic on Dart's hot restart
- Convenient project management using Python
- Managing windows on desktop platforms
- Writing user interface texts in the translation file

## Platform Support

Structuring a Flutter project that targets multiple platforms can be a challenging task, especially when incorporating Rust. With this template, you don't have to start from scratch or face the added complexity of integrating Rust.

- ✅ Windows: Tested and supported
- ✅ Linux: Tested and supported
- ✅ Android: Tested and supported
- ✅ macOS: Tested and supported
- ✅ iOS: Tested and supported
- ⏸️ Web: Not now [but considered](https://github.com/cunarist/flutter-rust-app-template/issues/34)

## Contribution

If you have any suggestions or want to report a bug, please leave it as an [issue](https://github.com/cunarist/flutter-rust-app-template/issues) or a [pull request](https://github.com/cunarist/flutter-rust-app-template/pulls). We will try to respond as quickly as possible.

# 🧱 Project Structure

**Flutter** deals with the cross-platform user interface while **Rust** handles the business logic. The front-end and back-end are completely separated, which means that Dart code and Rust code should be detachable from each other. These two worlds communicate through channels.

Moreover, you can conveniently receive the latest commits from [`flutter-rust-app-template`](https://github.com/cunarist/flutter-rust-app-template) into your repository using the provided Python script below.

# 👜 System Preparation

You can use an IDE of your choice. However, [Visual Studio Code](https://code.visualstudio.com/) is recommended because it has extensive support from Flutter and Rust communities.

## Basic Steps

1. Git: Go to the [official downloads page](https://git-scm.com/downloads)
1. Python: This is needed to automate development process. Download it from the app store if your system doesn't provide a pre-installed version. It's also available at [official downloads page](https://www.python.org/downloads/). Make sure `python` is available in the path environment variable. Version 3.11 or higher is recommended.
1. Rust: Refer to the [official docs](https://doc.rust-lang.org/book/ch01-01-installation.html). Version 1.69 or higher is recommended.
1. Flutter: Refer to the [official docs](https://docs.flutter.dev/get-started/install). Version 3.10 or higher is recommended.

And then you can make sure that your system is ready for development in the terminal.

```
git --version
python --version
rustc --version
flutter doctor
```

Read the output carefully and install the necessary components described in the terminal. You can repeat these commands to verify your system status while you are installing those components.

## Extra Steps

If you are planning to compile your code for Windows, Linux, or macOS, you can skip this section.

For Android, open up Android Studio and go to the `SDK Manager`. In the `SDK Tools` tab, enable the `NDK (side by side)` component. After that, run the following commands.

```
cargo install cargo-ndk
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

For iOS, run the following commands.

```
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
```

Setting up your system with extra build targets can sometimes present various issues. If you encounter any problems, feel free to visit [the discussions page](https://github.com/cunarist/flutter-rust-app-template/discussions) and open a Q&A thread for assistance. You can also refer to the `flutter_rust_bridge` docs for [instructions on installing the necessary components on your system](https://cjycode.com/flutter_rust_bridge/template/setup.html).

# 🗃️ Setting Up

Install the dependencies.

```
pip install -r requirements.txt
flutter pub get
rustup component add clippy
cargo install cargo-bloat
```

Generate configuration files or update them from template files if they already exist. Make sure to check the terminal output and fill in those files manually after the generation process is complete.

```
python automate config-filling
```

# 🍳 Actual Development

Check and fix problems in Python, Dart, and Rust code. For Rust, it checks the code in release mode.

```
python automate code-quality
```

Run the app in debug mode.

```
flutter run
```

Build the app in release mode.

```
flutter build (platform) --release
```

Check the actual sizes of compiled binaries in release mode.

```
python automate size-check (platform)
```

Set the app name and domain. This only works once and you cannot revert this.

```
python automate app-naming
```

Apply `app_icon_full.png` file in `./assets` to multiple platforms with [Flutter Launcher Icons](https://pub.dev/packages/flutter_launcher_icons). Appropriate rounding and scaling are applied per platform with the power of Python. On Linux, you should include the icon manually in the distribution package.

```
python automate icon-gen
```

Receive the latest commits from [`flutter-rust-app-template`](https://github.com/cunarist/flutter-rust-app-template).

```
python automate template-update
```

# ⛓️ MVVM Pattern

There are 3 layers composing this app.

- View is made up of Dart
- Viewmodel is provided by the bridge
- Model is made up of Rust

This system was designed to have minimal performance bottlenecks. Details are explained as comments in the actual code.

# 📁 Folder Structure

Basically, `./lib/main.dart` is the entry point of your Dart logic while `./native/hub/src/lib.rs` is the entry point of your Rust logic.

Most of the top-level folders come from the default Flutter template.

- `windows`: Platform-specific files
- `linux`: Platform-specific files
- `macos`: Platform-specific files
- `android`: Platform-specific files
- `ios`: Platform-specific files
- `web`: Platform-specific files
- `lib`: Dart modules empowering the Flutter application.

However, there are some extra folders created in `flutter-rust-app-template` to integrate other functionalities into development.

- `automate`: Python scripts for automating the development process. These scripts have nothing to do with the actual build and don't get included in the app release. Only for developers.
- `native`: The location of Rust library crates. Each crate inside this folder gets compiled into its own library binary(`.dll`/`.so`/`.dylib`).
- `assets`: A place for asset files such as images.

In addition, there might be some other temporary folders generated by tools or IDE you are using. Those should not be version-controlled.

# 📜 Rules

## The Front and the Back

Dart should only be used for the front-end user interface and Rust should handle all other back-end logic such as file handling, event handling, timer repetition, calculation, network communication, etc.

## Async over Threading

Use async functions. Don't use `std::thread`.

This template uses `tokio` to execute async functions. `tokio`'s async runtime allows millions of concurrent tasks to be run at the same time utilizing only the number of threads equivalent to the number of cores on the computer.

Refer to `tokio`'s [API docs](https://docs.rs/tokio/latest/tokio/) to be more familiar with its runtime and task management system.

## Internationalization

Always write user interface texts in `./assets/translations.csv`.

When an app gains popularity, there comes a need to support multiple human languages. However, manually replacing thousands of text widgets in the user interface is not a trivial task. Therefore it is a must to write texts that will be presented to normal users in the translation file.

Refer to [Easy Localization](https://pub.dev/packages/easy_localization) docs for more details.

## Guiding Comments

Please write kind and readable comments and also attach doc comments to important elements of your code.

You are probably not going to be developing on your own. Other developers should be able to get a grasp of the complex code that you wrote. Long and detailed comments are also welcomed.

## Development Automation

Rely on Python scripts in `./automate` for faster and easier development.

## Modification Restictions

Be careful all the time! You shouldn't edit files without enough knowledge of how they work. Below are the top-level files and folders that are allowed to edit during app development:

- `lib`: Dart modules.
  - Do not modify the `bridge` folder inside.
- `pubspec.yaml`: Dart settings and dependencies.
- `assets`: Asset files.
- `native`: Rust library crates.
  - Do not modify the `bridge` module inside the `hub` crate.
