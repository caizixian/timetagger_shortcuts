use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    key: String,
    t1: u64,
    t2: u64,
    ds: Option<String>,
    mt: u64,
    st: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Records {
    pub(crate) records: Vec<Record>,
}
