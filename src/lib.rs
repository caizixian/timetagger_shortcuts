mod api;
mod keygen;
mod records;

use std::time::{SystemTime, UNIX_EPOCH};
pub(crate) fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX EPOCH!")
        .as_secs()
}

pub use api::*;
pub use records::*;
