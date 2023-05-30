/// This module supports communication with Rust.
/// More specifically, sending user actions and
/// receiving viewmodel updates are supported.
/// This `wrapper.dart` includes everything you need,
/// so do not import anything else inside the `bridge` folder.
/// DO NOT EDIT.

import 'dart:async';
import 'bridge_definitions.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

export 'bridge_definitions.dart';

var viewmodelUpdateBroadcaster = StreamController<String>.broadcast();
var viewUpdateBroadcaster = StreamController<ViewUpdate>.broadcast();

Future<void> organizeRustRelatedThings() async {
  assert(() {
    // In debug mode, clean up the viewmodel upon Dart's hot restart.
    api.cleanViewmodel();
    return true;
  }());
  var endpointsOnRustThread = api.prepareChannels();
  endpointsOnRustThread.move = true;
  await api.layEndpointsOnRustThread(rustOpaque: endpointsOnRustThread);
  var viewmodelUpdateStream = api.prepareViewmodelUpdateStream();
  viewmodelUpdateStream.listen((event) {
    viewmodelUpdateBroadcaster.add(event);
  });
  var viewUpdateStream = api.prepareViewUpdateStream();
  viewUpdateStream.listen((event) {
    viewUpdateBroadcaster.add(event);
  });
  await Future.delayed(const Duration(milliseconds: 100));
  api.startRustLogic();
}

Serialized? readViewmodel(String itemAddress) {
  Serialized? serialized = api.readViewmodel(
    itemAddress: itemAddress,
  );
  return serialized;
}

void sendUserAction(String taskAddress, Serialized serialized) {
  api.sendUserAction(
    taskAddress: taskAddress,
    serialized: serialized,
  );
}
