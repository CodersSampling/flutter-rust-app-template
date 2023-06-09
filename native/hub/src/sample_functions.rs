//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::Serialized;
use crate::bridge::update_viewmodel;
use crate::data_model;
use crate::messages;
use prost::Message;
use tokio::task::spawn_blocking;

pub async fn calculate_something(serialized: Serialized) {
    let buffer = serialized.bytes.as_slice();
    let received = messages::basic::SampleLetter::decode(buffer).unwrap();
    println!("{:?}", received.letter);
    let key = "someValueCategory.thisNumber";
    let mut hashmap = data_model::SAMPLE_COLLECTION.write().await;
    if !hashmap.contains_key(key) {
        hashmap.insert(String::from(key), 0);
    }
    let value = *hashmap.get(key).unwrap();
    let new_value = sample_crate::add_seven(value);
    hashmap.insert(String::from(key), new_value);
    let serialized = Serialized {
        // These message structs generated from Protobuf files
        // will be used often.
        bytes: messages::basic::SampleNumber {
            value: new_value,
            dummy_one: 1,
            dummy_two: 2,
            dummy_three: vec![3, 4, 5],
        }
        .encode_to_vec(),
        // `formula` is a customized promise between the front and the back
        // that is useful for clarifying the serialization method
        // of the bytes data.
        formula: String::from("exchange/protobuf"),
    };
    // In Rust, you update the viewmodel with
    // `update_viewmodel` function imported from module `bridge`.
    // Because Dart widgets are bound to the viewmodel items,
    // updating them from Rust will automatically trigger
    // related Dart widgets to be rebuilt.
    update_viewmodel("someItemCategory.count", serialized);
}

pub async fn keep_drawing_mandelbrot() {
    let mut scale: f64 = 1.0;
    loop {
        // Never use `std::thread::sleep` in `tokio`'s core threads
        // because it will block the async runtime.
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        scale *= 0.95;
        if scale < 1e-7 {
            scale = 1.0
        };
        // Because drawing a mandelbrot image is
        // a CPU-intensive blocking task,
        // we use `spawn_blocking` instead of `spawn`
        // to delegate this task to `tokio`'s blocking threads.
        // In real-world async scenarios,
        // thread blocking tasks that take more than 10 microseconds
        // are considered better to be sent to an outer thread.
        let join_handle = spawn_blocking(move || {
            sample_crate::mandelbrot(
                sample_crate::Size {
                    width: 64,
                    height: 64,
                },
                sample_crate::Point {
                    x: 0.360,
                    y: -0.641,
                },
                scale,
                4,
            )
            .unwrap()
        });
        let calculated = join_handle.await;
        if let Ok(mandelbrot) = calculated {
            let payload = Serialized {
                bytes: mandelbrot,
                formula: String::from("image/png"),
            };
            update_viewmodel("someItemCategory.mandelbrot", payload);
        }
    }
}

pub async fn keep_adding_one() {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let key = "someValueCategory.thisNumber";
        let mut hashmap = data_model::SAMPLE_COLLECTION.write().await;
        if !hashmap.contains_key(key) {
            hashmap.insert(String::from(key), 0);
        }
        let value = *hashmap.get(key).unwrap();
        let new_value = value + 1;
        hashmap.insert(String::from(key), new_value);
        let serialized = Serialized {
            bytes: messages::basic::SampleNumber {
                value: new_value,
                dummy_one: 1,
                dummy_two: 2,
                dummy_three: vec![3, 4, 5],
            }
            .encode_to_vec(),
            formula: String::from("exchange/protobuf"),
        };
        update_viewmodel("someItemCategory.count", serialized);
    }
}
