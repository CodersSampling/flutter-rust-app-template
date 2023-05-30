import 'package:universal_platform/universal_platform.dart';
import 'package:flutter/material.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:easy_localization_loader/easy_localization_loader.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'app.dart';
import 'constants.dart';
import 'bridge/wrapper.dart';

/// There are 2 threads behind this app, one for Dart and one for Rust.
/// This `main` function is the entry point for the Dart logic,
/// which occupies one of those 2 threads.
void main() async {
  // Make the Rust side ready.
  await organizeRustRelatedThings();

  // Initialize packages.
  WidgetsFlutterBinding.ensureInitialized();
  await EasyLocalization.ensureInitialized();

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

  // Set desktop window shape.
  if (UniversalPlatform.isWindows ||
      UniversalPlatform.isLinux ||
      UniversalPlatform.isMacOS) {
    doWhenWindowReady(() {
      appWindow.minSize = minimumSize;
      appWindow.size = initialSize;
      appWindow.alignment = Alignment.center;
      appWindow.show();
    });
  }
}
