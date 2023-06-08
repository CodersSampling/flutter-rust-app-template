use tokio::task::spawn;

mod bridge;
mod data_model;
mod messages;
mod sample_functions;
mod with_user_action;

/// Dart operates within a single thread, while Rust has multiple threads.
/// This `main` function is the entry point for the Rust logic.
/// `tokio`'s async runtime is used for concurrency.
/// Always use non-blocking async functions in `tokio`'s core threads,
/// such as `tokio::time::sleep` or `tokio::fs::File::open`.
#[tokio::main]
async fn main() {
    // This is `tokio::sync::mpsc::Reciver` that receives user actions in an async manner.
    let mut user_action_receiver = bridge::get_user_action_receiver();
    // These are used for telling the tasks to stop running.
    let (shutdown_signal_sender, shutdown_signal_receiver) = tokio::sync::oneshot::channel();
    let root_join_handle = spawn(async move {
        // Repeat `tokio::task::spawn` anywhere in your code
        // if more concurrent tasks are needed.
        spawn(sample_functions::keep_drawing_mandelbrot());
        spawn(sample_functions::keep_adding_one());
        while let Some(user_action) = user_action_receiver.recv().await {
            spawn(with_user_action::handle_user_action(user_action));
        }
        // Send the shutdown signal after the user action channel is closed,
        // which is typically triggered by Dart's hot restart.
        shutdown_signal_sender.send(()).ok();
    });
    // Begin the tasks and terminate them upon receiving the shutdown signal
    tokio::select! {
        _ = root_join_handle => {}
        _ = shutdown_signal_receiver => {}
    }
    // In debug mode, clean up the data upon Dart's hot restart
    #[cfg(debug_assertions)]
    {
        data_model::clean_model().await;
    }
}
