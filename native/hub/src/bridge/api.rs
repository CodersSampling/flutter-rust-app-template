use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::SyncReturn;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct Serialized {
    pub bytes: Vec<u8>,
    pub formula: String,
}

#[derive(Clone)]
pub struct ViewUpdate {
    pub display_address: String,
    pub serialized: Serialized,
}

pub struct ViewmodelUpdate {
    pub item_address: String,
    pub serialized: Serialized,
}

pub struct UserAction {
    pub task_address: String,
    pub serialized: Serialized,
}

type Cell<T> = RefCell<Option<T>>;
type SharedCell<T> = Arc<Mutex<Cell<T>>>;

type Viewmodel = HashMap<String, Serialized>;
type ViewmodelUpdateStream = StreamSink<String>;
type ViewUpdateStream = StreamSink<ViewUpdate>;
type UserActionSender = Sender<UserAction>;
type UserActionReceiver = Receiver<UserAction>;
type ViewmodelUpdateSender = Sender<ViewmodelUpdate>;
type ViewmodelUpdateReceiver = Receiver<ViewmodelUpdate>;

// For thread 0 running Dart
thread_local! {
    pub static VIEWMODEL: Cell<Viewmodel> = RefCell::new(Some(HashMap::new()));
    pub static USER_ACTION_SENDER: Cell<UserActionSender> = RefCell::new(None);
    pub static VIEWMODEL_UPDATE_RECEIVER: Cell<ViewmodelUpdateReceiver> =RefCell::new(None);

}

// For thread 1~N running Rust
thread_local! {
    pub static ARE_CHANNELS_READY: RefCell<bool> = RefCell::new(false);
    pub static VIEWMODEL_UPDATE_SENDER: Cell<ViewmodelUpdateSender>= RefCell::new(None);
    pub static VIEWMODEL_UPDATE_STREAM: Cell<ViewmodelUpdateStream> = RefCell::new(None);
    pub static VIEW_UPDATE_STREAM: Cell<ViewUpdateStream> = RefCell::new(None);
}

// For sharing between threads
lazy_static! {
    pub static ref VIEWMODEL_UPDATE_STREAM_SHARED: SharedCell<ViewmodelUpdateStream> =
        Arc::new(Mutex::new(RefCell::new(None)));
    pub static ref VIEW_UPDATE_STREAM_SHARED: SharedCell<ViewUpdateStream> =
        Arc::new(Mutex::new(RefCell::new(None)));
    pub static ref USER_ACTION_RECEIVER_SHARED: SharedCell<UserActionReceiver> =
        Arc::new(Mutex::new(RefCell::new(None)));
    pub static ref VIEWMODEL_UPDATE_SENDER_SHARED: SharedCell<ViewmodelUpdateSender> =
        Arc::new(Mutex::new(RefCell::new(None)));
}

pub fn prepare_viewmodel_update_stream(viewmodel_update_stream: StreamSink<String>) {
    // Thread 1 running Rust
    let cell = VIEWMODEL_UPDATE_STREAM_SHARED.lock().unwrap();
    cell.replace(Some(viewmodel_update_stream));
}

pub fn prepare_view_update_stream(view_update_stream: StreamSink<ViewUpdate>) {
    // Thread 1 running Rust
    let cell = VIEW_UPDATE_STREAM_SHARED.lock().unwrap();
    cell.replace(Some(view_update_stream));
}

pub fn prepare_channels() -> SyncReturn<()> {
    // Thread 0 running Dart
    let (user_action_sender, user_action_receiver) = channel(1024);
    USER_ACTION_SENDER.with(move |inner| {
        inner.replace(Some(user_action_sender));
    });
    let cell = USER_ACTION_RECEIVER_SHARED.lock().unwrap();
    cell.replace(Some(user_action_receiver));
    let (viewmodel_update_sender, viewmodel_update_receiver) = channel(1024);
    VIEWMODEL_UPDATE_RECEIVER.with(move |inner| {
        inner.replace(Some(viewmodel_update_receiver));
    });
    let cell = VIEWMODEL_UPDATE_SENDER_SHARED.lock().unwrap();
    cell.replace(Some(viewmodel_update_sender));
    SyncReturn(())
}

pub fn start_rust_logic() {
    // Thread 1 running Rust
    crate::main();
}

pub fn send_user_action(task_address: String, serialized: Serialized) -> SyncReturn<()> {
    // Thread 0 running Dart
    USER_ACTION_SENDER.with(move |inner| {
        let borrowed = inner.borrow();
        let sender = borrowed.as_ref().unwrap();
        let user_action = UserAction {
            task_address,
            serialized,
        };
        sender.try_send(user_action).ok();
    });
    SyncReturn(())
}

/// This function is meant to be called when Dart's hot restart is triggered in debug mode.
pub fn clean_viewmodel() -> SyncReturn<()> {
    // Thread 0 running Dart
    VIEWMODEL.with(move |inner| {
        inner.replace(Some(HashMap::new()));
    });
    SyncReturn(())
}

pub fn read_viewmodel(item_address: String) -> SyncReturn<Option<Serialized>> {
    // Thread 0 running Dart
    let item_option = VIEWMODEL_UPDATE_RECEIVER.with(move |inner| {
        let receiver_cell = inner;
        let mut receiver = receiver_cell.replace(None).unwrap();
        VIEWMODEL.with(move |inner| {
            let mut borrowed = inner.borrow_mut();
            let hashmap = borrowed.as_mut().unwrap();
            while let Ok(viewmodel_update) = receiver.try_recv() {
                hashmap.insert(viewmodel_update.item_address, viewmodel_update.serialized);
            }
            receiver_cell.replace(Some(receiver));
            hashmap.get(&item_address).cloned()
        })
    });
    SyncReturn(item_option)
}
