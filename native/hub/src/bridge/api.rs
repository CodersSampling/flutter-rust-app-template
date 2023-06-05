use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::SyncReturn;
use lazy_static::lazy_static;
use std::cell::RefCell;
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

type ViewmodelUpdateStream = StreamSink<ViewmodelUpdate>;
type UserActionSender = Sender<UserAction>;
type UserActionReceiver = Receiver<UserAction>;

// For thread 0 running Dart
thread_local! {
    pub static USER_ACTION_SENDER: Cell<UserActionSender> = RefCell::new(None);
}

// For thread 1~N running Rust
thread_local! {
    pub static VIEWMODEL_UPDATE_STREAM: Cell<ViewmodelUpdateStream> = RefCell::new(None);
}

// For sharing between threads
lazy_static! {
    pub static ref VIEWMODEL_UPDATE_STREAM_SHARED: SharedCell<ViewmodelUpdateStream> =
        Arc::new(Mutex::new(RefCell::new(None)));
    pub static ref USER_ACTION_RECEIVER_SHARED: SharedCell<UserActionReceiver> =
        Arc::new(Mutex::new(RefCell::new(None)));
}

pub fn prepare_viewmodel_update_stream(viewmodel_update_stream: StreamSink<ViewmodelUpdate>) {
    // Thread 1 running Rust
    let cell = VIEWMODEL_UPDATE_STREAM_SHARED.lock().unwrap();
    cell.replace(Some(viewmodel_update_stream));
}

pub fn prepare_channels() -> SyncReturn<()> {
    // Thread 0 running Dart
    let (user_action_sender, user_action_receiver) = channel(1024);
    USER_ACTION_SENDER.with(move |inner| {
        inner.replace(Some(user_action_sender));
    });
    let cell = USER_ACTION_RECEIVER_SHARED.lock().unwrap();
    cell.replace(Some(user_action_receiver));
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
