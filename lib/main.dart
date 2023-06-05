import 'package:universal_platform/universal_platform.dart';
import 'package:flutter/material.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:easy_localization_loader/easy_localization_loader.dart';
import 'package:window_manager/window_manager.dart';
import 'app.dart';
import 'constants.dart';
import 'bridge/wrapper.dart';

/// Dart operates within a single thread, while Rust has multiple threads.
/// This `main` function is the entry point for the Dart logic.
void main() async {
  // Make the Rust side ready.
  await organizeRustRelatedThings();

  // Initialize packages.
  WidgetsFlutterBinding.ensureInitialized();
  await EasyLocalization.ensureInitialized();
  await windowManager.ensureInitialized();

  // Set desktop window shape.
  if (UniversalPlatform.isWindows ||
      UniversalPlatform.isLinux ||
      UniversalPlatform.isMacOS) {
    WindowOptions windowOptions = const WindowOptions(
      size: initialSize,
      minimumSize: minimumSize,
      center: true,
    );
    windowManager.waitUntilReadyToShow(windowOptions, () async {
      await windowManager.show();
    });
  }

  // Run everything.
  runApp(
    EasyLocalization(
      supportedLocales: const [
        Locale('en', 'US'),
        Locale('ko', 'KR'),
      ],
      path: 'assets/translations.csv',
      assetLoader: CsvAssetLoader(),
      fallbackLocale: const Locale('en', 'US'),
      child: const App(),
    ),
  );
}
