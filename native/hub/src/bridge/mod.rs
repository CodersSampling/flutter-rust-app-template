//! This module communicates with Dart.
//! More specifically, receiveing user actions and
//! sending viewmodel updates are supported.
//! DO NOT EDIT.

use api::Serialized;
use tokio::sync::mpsc::Receiver;

pub mod api;
mod bridge_generated;

fn check_and_initialize() {
    api::ARE_CHANNELS_READY.with(|inner| {
        let mut are_channels_ready = inner.borrow_mut();
        if !*are_channels_ready {
            api::VIEWMODEL_UPDATE_SENDER.with(|inner| {
                let cell = api::VIEWMODEL_UPDATE_SENDER_SHARED.lock().unwrap();
                let new = cell.borrow().as_ref().unwrap().clone();
                inner.replace(Some(new));
            });
            api::VIEWMODEL_UPDATE_STREAM.with(|inner| {
                let cell = api::VIEWMODEL_UPDATE_STREAM_SHARED.lock().unwrap();
                let new = cell.borrow().as_ref().unwrap().clone();
                inner.replace(Some(new));
            });
            api::VIEW_UPDATE_STREAM.with(|inner| {
                let cell = api::VIEW_UPDATE_STREAM_SHARED.lock().unwrap();
                let new = cell.borrow().as_ref().unwrap().clone();
                inner.replace(Some(new));
            });
            *are_channels_ready = true;
        }
    });
}

/// Updating the viewmodel will
/// automatically send a stream signal to Flutter widgets
/// which would trigger the rebuild.
/// `item_address` would be something like `someItemAddress.someName`.
/// After the serialized bytes are saved in the viewmodel,
/// it will be copied in memory when being read from the Dart side.
#[allow(dead_code)]
pub fn update_viewmodel(item_address: &str, serialized: Serialized) {
    check_and_initialize();
    api::VIEWMODEL_UPDATE_SENDER.with(|inner| {
        let borrowed = inner.borrow();
        let sender = borrowed.as_ref().unwrap();
        let viewmodel_update = api::ViewmodelUpdate {
            item_address: String::from(item_address),
            serialized,
        };
        sender.try_send(viewmodel_update).ok();
    });
    api::VIEWMODEL_UPDATE_STREAM.with(|inner| {
        let borrowed = inner.borrow();
        let stream = borrowed.as_ref().unwrap();
        stream.add(item_address.to_string());
    });
}

/// This function should only be used for
/// big data such as high-resolution images
/// that would take considerable amount of time
/// to be copied in memory.
/// This function doesn't involve any memory copy,
/// but the data will be gone once it is dropped by Dart.
/// Do not use this function for sending small data to the view.
/// In order to achieve proper MVVM pattern,
/// you should use `update_viewmodel` in most cases.
#[allow(dead_code)]
pub fn update_view(display_address: &str, serialized: Serialized) {
    check_and_initialize();
    #[cfg(debug_assertions)]
    if serialized.bytes.len() < 2_usize.pow(10) {
        println!(concat!(
            "You shouldn't use `update_view` unless it's a huge data. ",
            "Use `update_viewmodel` instead."
        ));
        return;
    }
    api::VIEW_UPDATE_STREAM.with(|inner| {
        let borrowed = inner.borrow();
        let stream = borrowed.as_ref().unwrap();
        let view_update = api::ViewUpdate {
            display_address: String::from(display_address),
            serialized,
        };
        stream.add(view_update);
    });
}

/// This function is expected to be used only once
/// during the initialization of the Rust logic.
pub fn get_user_action_receiver() -> Receiver<api::UserAction> {
    let cell = api::USER_ACTION_RECEIVER_SHARED.lock().unwrap();
    let option = cell.replace(None);
    option.unwrap()
}
