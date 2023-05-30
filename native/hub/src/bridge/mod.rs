//! This module communicates with Dart.
//! More specifically, receiveing user actions and
//! sending viewmodel updates are supported.
//! DO NOT EDIT.

use api::Serialized;
use ref_thread_local::RefThreadLocal;
use tokio::sync::mpsc::Receiver;

pub mod api;
mod bridge_generated;

/// Updating the viewmodel will
/// automatically send a stream signal to Flutter widgets
/// which would trigger the rebuild.
/// `item_address` would be something like `someItemAddress.someName`.
/// After the serialized bytes are saved in the viewmodel,
/// it will be copied in memory when being read from the Dart side.
#[allow(dead_code)]
pub fn update_viewmodel(item_address: &str, serialized: Serialized) {
    let refcell = api::VIEWMODEL_UPDATE_SENDER.borrow();
    let borrowed = refcell.borrow();
    let option = borrowed.as_ref();
    if let Some(sender) = option {
        let viewmodel_update = api::ViewmodelUpdate {
            item_address: String::from(item_address),
            serialized,
        };
        sender.try_send(viewmodel_update).ok();
    }
    let refcell = api::VIEWMODEL_UPDATE_STREAM.borrow();
    let borrowed = refcell.borrow();
    let option = borrowed.as_ref();
    if let Some(stream) = option {
        stream.add(item_address.to_string());
    }
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
    #[cfg(debug_assertions)]
    if serialized.bytes.len() < 2_usize.pow(10) {
        println!(concat!(
            "You shouldn't use `update_view` unless it's a huge data. ",
            "Use `update_viewmodel` instead."
        ));
        return;
    }
    let view_update = api::ViewUpdate {
        display_address: String::from(display_address),
        serialized,
    };
    let refcell = api::VIEW_UPDATE_STREAM.borrow();
    let borrowed = refcell.borrow();
    let option = borrowed.as_ref();
    if let Some(stream) = option {
        stream.add(view_update);
    }
}

/// This function is expected to be used only once
/// during the initialization of the Rust logic.
pub fn get_user_action_receiver() -> Receiver<api::UserAction> {
    let refcell = api::USER_ACTION_RECEIVER.borrow();
    let option = refcell.replace(None);
    option.expect("User action receiver does not exist!")
}
