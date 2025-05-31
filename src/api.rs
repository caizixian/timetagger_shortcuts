use crate::{Record, Records, get_timestamp};
use anyhow::Result;
use std::path::Path;

pub struct APIClient {
    base_url: String,
    authtoken: String,
}

impl APIClient {
    pub fn new(base_url: String, authtoken: String) -> APIClient {
        APIClient {
            base_url,
            authtoken,
        }
    }

    pub fn client_from_file<T: AsRef<Path>>(path: T) -> APIClient {
        let mut authtoken = None;
        let mut base_url = None;
        for line in std::fs::read_to_string(path).unwrap().lines() {
            let mut parts = line.split("=");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            match key {
                "BASE_URL" => {
                    base_url = Some(value.to_string());
                }
                "AUTHTOKEN" => {
                    authtoken = Some(value.to_string());
                }
                _ => {
                    panic!("Invalid key in .env")
                }
            }
        }
        APIClient::new(base_url.unwrap(), authtoken.unwrap())
    }

    pub fn get_records(&self, from: u64, to: u64) -> Result<Vec<Record>> {
        let client = reqwest::blocking::ClientBuilder::new()
            .use_rustls_tls()
            .build()
            .unwrap();
        let res = client
            .get(format!(
                "{}/records?timerange={}-{}",
                self.base_url, from, to
            ))
            .header("authtoken", &self.authtoken)
            .send()?;
        let records: Records = res.json()?;
        Ok(records.records)
    }

    pub fn get_running_records(&self) -> Result<Vec<Record>> {
        let now = get_timestamp();
        let lower_bound = now - 35 * 60;
        let upper_bound = now + 60;
        self.get_records(lower_bound, upper_bound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client_from_env() -> APIClient {
        APIClient::client_from_file(".env")
    }
}
