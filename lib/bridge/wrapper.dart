/// This module supports communication with Rust.
/// More specifically, sending user actions and
/// receiving viewmodel updates are supported.
/// This `wrapper.dart` includes everything you need,
/// so do not import anything else inside the `bridge` folder.
/// DO NOT EDIT.

import 'dart:async';
import 'dart:collection';
import 'bridge_definitions.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

export 'bridge_definitions.dart';

var viewmodel = HashMap<String, Serialized>();
var viewmodelUpdateBroadcaster = StreamController<String>.broadcast();

Future<void> organizeRustRelatedThings() async {
  api.prepareChannels();
  var viewmodelUpdateStream = api.prepareViewmodelUpdateStream();
  viewmodelUpdateStream.listen((event) {
    viewmodel[event.itemAddress] = event.serialized;
    viewmodelUpdateBroadcaster.add(event.itemAddress);
  });
  await Future.delayed(const Duration(milliseconds: 100));
  api.startRustLogic();
}

void sendUserAction(String taskAddress, Serialized serialized) {
  api.sendUserAction(
    taskAddress: taskAddress,
    serialized: serialized,
  );
}
