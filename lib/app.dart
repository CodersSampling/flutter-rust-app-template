import 'package:flutter/material.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:universal_platform/universal_platform.dart';
import 'package:bitsdojo_window/bitsdojo_window.dart';
import 'constants.dart';
import 'home.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    context.setLocale(const Locale('en', 'US'));

    // Return the actual app structure.
    return MaterialApp(
      onGenerateTitle: (context) {
        if (UniversalPlatform.isWindows ||
            UniversalPlatform.isLinux ||
            UniversalPlatform.isMacOS) {
          appWindow.title = 'appTitle'.tr(); // For desktop
        }
        return 'appTitle'.tr(); // For mobile and web
      },
      theme: ThemeData(
        colorScheme: const ColorScheme.light(
          primary: primaryColor,
          secondary: secondaryColor,
        ),
      ),
      darkTheme: ThemeData(
        colorScheme: const ColorScheme.dark(
          primary: primaryColor,
          secondary: secondaryColor,
        ),
      ),
      themeMode: ThemeMode.system,
      home: const Home(),
      localizationsDelegates: context.localizationDelegates,
      supportedLocales: context.supportedLocales,
      locale: context.locale,
    );
  }
}
