use tokio::task::spawn_local;

mod bridge;
mod data_model;
mod sample_functions;
mod with_user_action;

/// There are 2 threads behind this app, one for Dart and one for Rust.
/// This `main` function is the entry point for the Rust logic,
/// which occupies one of those 2 threads.
/// `tokio`'s runtime is used for single-threaded async concurrency.
/// Avoid using a threadpool or GPU unless more computing power is needed.
/// Always use non-blocking async functions on the main thread, such as `tokio::time::sleep`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // This is `tokio::sync::mpsc::Reciver` that receives user actions in an async manner.
    let mut user_action_receiver = bridge::get_user_action_receiver();
    // These are used for telling the tasks to stop running.
    let (shutdown_signal_sender, shutdown_signal_receiver) = tokio::sync::oneshot::channel();
    // By using `tokio::task::LocalSet`, all tasks are ensured
    // that they are executed on the main thread.
    let local_set = tokio::task::LocalSet::new();
    local_set.spawn_local(async move {
        // Repeat `tokio::task::spawn_local` anywhere in your code
        // if more concurrent tasks are needed.
        // Always use `tokio::task::spawn_local` over `tokio::task::spawn`
        // to guarantee that the new task is executed on the main thread.
        // Also, `tokio::task::spawn_local` doesn't require the parameter
        // to have `Send` trait, unlike `tokio::task::spawn`.
        spawn_local(sample_functions::keep_drawing_mandelbrot());
        while let Some(user_action) = user_action_receiver.recv().await {
            spawn_local(with_user_action::handle_user_action(user_action));
        }
        // Send the shutdown signal after the user action channel is closed,
        // which is typically triggered by Dart's hot restart.
        shutdown_signal_sender.send(()).ok();
    });
    // Begin the tasks and terminate them upon receiving the shutdown signal
    tokio::select! {
        _ = local_set => {}
        _ = shutdown_signal_receiver => {}
    }
    // In debug mode, clean up the data upon Dart's hot restart
    #[cfg(debug_assertions)]
    {
        data_model::clean_model();
    }
}
