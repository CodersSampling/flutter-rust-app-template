//! This module is the model part of MVVM pattern.
//! To be more clear, this module was named `data_model` instead of just `model`.

use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// This function is meant to be called when Dart's hot restart is triggered in debug mode.
#[cfg(debug_assertions)]
pub async fn clean_model() {
    *SAMPLE_NUMBERS.write().await = HashMap::new();
}

lazy_static! {
    pub static ref SAMPLE_NUMBERS: RwLock<HashMap<String, i32>> = RwLock::new(HashMap::new());
}
