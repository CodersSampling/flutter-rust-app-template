//! This module communicates with Dart.
//! More specifically, receiveing user actions and
//! sending viewmodel updates are supported.
//! DO NOT EDIT.

use api::Serialized;
use tokio::sync::mpsc::Receiver;

pub mod api;
mod bridge_generated;

/// Updating the viewmodel will
/// automatically send a stream signal to Flutter widgets
/// and trigger the rebuild.
/// No memory copy is involved as the bytes are moved directly to Dart,
/// thanks to the `zero-copy` feature of `flutter_rust_bridge`.
#[allow(dead_code)]
pub fn update_viewmodel(item_address: &str, serialized: Serialized) {
    api::VIEWMODEL_UPDATE_STREAM.with(|inner| {
        let mut borrowed = inner.borrow_mut();
        let option = borrowed.as_ref();
        let viewmodel_update = api::ViewmodelUpdate {
            item_address: String::from(item_address),
            serialized,
        };
        if let Some(stream) = option {
            stream.add(viewmodel_update);
        } else {
            let cell = api::VIEWMODEL_UPDATE_STREAM_SHARED.lock().unwrap();
            let stream = cell.borrow().as_ref().unwrap().clone();
            stream.add(viewmodel_update);
            borrowed.replace(stream);
        }
    });
}

/// This function is expected to be used only once
/// during the initialization of the Rust logic.
pub fn get_user_action_receiver() -> Receiver<api::UserAction> {
    let cell = api::USER_ACTION_RECEIVER_SHARED.lock().unwrap();
    let option = cell.replace(None);
    option.unwrap()
}
