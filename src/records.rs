use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::get_timestamp;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    key: String,
    t1: u64,
    t2: u64,
    ds: Option<String>,
    mt: u64,
    st: Option<f64>,
}

static KEY_LENGTH: usize = 8;

impl Record {
    pub fn new(ds: String) -> Self {
        let now = get_timestamp();
        Record {
            key: crate::generate_key(KEY_LENGTH),
            t1: now,
            t2: now,
            mt: now,
            ds: Some(ds),
            st: Some(0.0),
        }
    }

    pub fn is_running(&self) -> bool {
        self.t1 == self.t2
    }

    pub fn stop(self) -> Record {
        let now = get_timestamp();
        Record {
            t2: now,
            mt: now,
            ..self
        }
    }

    pub fn str_to_tags(ds: &str) -> HashSet<String> {
        let parts = ds.split(" ");

        HashSet::from_iter(
            parts
                .filter(|part| part.starts_with("#"))
                .map(|part| part.to_string()),
        )
    }

    pub fn tags(&self) -> HashSet<String> {
        let ds = self.ds.as_deref().unwrap_or_default();
        Self::str_to_tags(ds)
    }
}
