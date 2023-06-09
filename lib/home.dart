import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:easy_localization/easy_localization.dart';
import 'bridge/wrapper.dart';
import 'messages/basic.pb.dart';

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
              // For better performance, filter signals
              // by checking `itemAddress` with the `where` method.
              // This approach allows the builder to rebuild its widget
              // only when there are changes
              // to the specific viewmodel item it is interested in.
              stream: viewmodelUpdateBroadcaster.stream.where((itemAdress) {
                return itemAdress == 'someItemCategory.mandelbrot';
              }),
              builder: (context, snapshot) {
                var itemAddress = 'someItemCategory.mandelbrot';
                // `viewmodel` is inside `bridge/wrapper.dart`
                var serialized = viewmodel[itemAddress];
                // If the app has just started
                // and Rust didn't update the viewmodel yet,
                // the item's value will be null.
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
                var serialized = viewmodel[itemAddress];
                if (serialized == null) {
                  return Text('counter.blankText'.tr());
                } else {
                  // Unpack serialized Protobuf bytes data.
                  var message = SampleNumber.fromBuffer(serialized.bytes);
                  var number = message.value;
                  return Text('counter.informationText'.tr(namedArgs: {
                    'theValue': number.toString(),
                  }));
                }
              },
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          var serialized = Serialized(
            // These message classes generated from Protobuf files
            // will be used often.
            bytes: SampleLetter(
                letter: 'Hello from Dart!',
                dummyOne: 1,
                dummyTwo: 2,
                dummyThree: [3, 4, 5]).writeToBuffer(),
            formula: 'exchange/protobuf',
          );
          // Use `sendUserAction` from `bridge/wrapper.dart`
          // to send the user action to Rust.
          sendUserAction(
            'someTaskCategory.calculateSomething',
            serialized,
          );
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
