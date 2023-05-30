//! This module runs the corresponding function
//! with the received user action.

use crate::bridge::api::UserAction;
use crate::sample_functions;

pub async fn handle_user_action(user_action: UserAction) {
    // `task_address` would be something like "addressCategory.someTask"
    let task_address = user_action.task_address;
    let serialized = user_action.serialized;
    let layered: Vec<&str> = task_address.split('.').collect();
    if layered.is_empty() {
    } else if layered[0] == "someTaskCategory" {
        if layered.len() == 1 {
        } else if layered[1] == "calculateSomething" {
            sample_functions::calculate_something(serialized).await;
        }
    }
}
