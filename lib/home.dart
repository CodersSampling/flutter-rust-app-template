import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:messagepack/messagepack.dart';
import 'bridge/wrapper.dart';

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            // `StreamBuilder` listens to a stream
            // and rebuilds the widget accordingly.
            StreamBuilder<String>(
              // Receive viewmodel update signals from Rust
              // with `viewmodelUpdateBroadcaster` from `bridge/wrapper.dart`,
              // For better performance, filters signals
              // by checking `itemAddress` with the `where` method.
              // This approach allows the builder to rebuild its widget
              // only when there are changes
              // to the specific viewmodel item it is interested in.
              stream: viewmodelUpdateBroadcaster.stream.where((itemAdress) {
                return itemAdress == 'someItemCategory.mandelbrot';
              }),
              builder: (context, snapshot) {
                var itemAddress = 'someItemCategory.mandelbrot';
                // Use `readViewmodel` from `bridge/wrapper.dart`
                // to read the viewmodel item made from Rust.
                var serialized = readViewmodel(itemAddress);
                // If the app has just started
                // and Rust didn't update the viewmodel item yet,
                // the returned value will be null.
                if (serialized == null) {
                  return Container(
                    margin: const EdgeInsets.all(20),
                    width: 256,
                    height: 256,
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(8.0),
                      color: Colors.black,
                    ),
                  );
                } else {
                  Uint8List imageData = serialized.bytes;
                  return Container(
                    margin: const EdgeInsets.all(20),
                    width: 256,
                    height: 256,
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(8.0),
                    ),
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(8),
                      child: FittedBox(
                        fit: BoxFit.contain,
                        child: Image.memory(
                          imageData,
                          width: 64,
                          height: 64,
                          gaplessPlayback: true,
                        ),
                      ),
                    ),
                  );
                }
              },
            ),
            StreamBuilder<String>(
              stream: viewmodelUpdateBroadcaster.stream.where((itemAddress) {
                return itemAddress == 'someItemCategory.count';
              }),
              builder: (context, snapshot) {
                var itemAddress = 'someItemCategory.count';
                var serialized = readViewmodel(itemAddress);
                if (serialized == null) {
                  return Text('counter.blankText'.tr());
                } else {
                  // Unpack serialized Messagepack bytes data.
                  var tree = Unpacker.fromList(serialized.bytes).unpackMap();
                  String numberText = tree['value'].toString();
                  return Text('counter.informationText'.tr(namedArgs: {
                    'theValue': numberText,
                  }));
                }
              },
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          Serialized payload = Serialized(
            bytes: Uint8List(0),
            formula: 'none',
          );
          // Use `sendUserAction` from `bridge/wrapper.dart`
          // to send the user action to Rust.
          sendUserAction(
            'someTaskCategory.calculateSomething',
            payload,
          );
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
