use crate::{Record, get_timestamp};
use anyhow::Result;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct RecordsResp {
    pub records: Vec<Record>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordPutResp {
    pub accepted: Vec<String>,
    pub failed: Vec<String>,
    pub errors: Vec<String>,
}

pub struct APIClient {
    base_url: String,
    client: Client,
}

pub struct EnvFile {
    pub base_url: Option<String>,
    pub authtoken: Option<String>,
}
impl EnvFile {
    pub fn from<T: AsRef<Path>>(path: T) -> EnvFile {
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
        EnvFile {
            base_url,
            authtoken,
        }
    }
}
impl APIClient {
    pub fn new(base_url: String, authtoken: String) -> Result<APIClient> {
        let mut headers = header::HeaderMap::new();
        headers.insert("authtoken", authtoken.parse()?);

        let client = reqwest::ClientBuilder::new()
            .use_rustls_tls()
            .default_headers(headers)
            .build()?;

        Ok(APIClient { base_url, client })
    }

    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<APIClient> {
        let env_file = EnvFile::from(path);
        APIClient::new(env_file.base_url.unwrap(), env_file.authtoken.unwrap())
    }

    pub async fn get_records(&self, from: u64, to: u64) -> Result<Vec<Record>> {
        let res = self
            .client
            .get(format!(
                "{}/records?timerange={}-{}",
                self.base_url, from, to
            ))
            .send()
            .await?;
        let records: RecordsResp = res.json().await?;
        Ok(records.records)
    }

    pub async fn get_running_records(&self) -> Result<Vec<Record>> {
        let now = get_timestamp();
        let lower_bound = now - 35 * 60;
        let upper_bound = now + 60;
        self.get_records(lower_bound, upper_bound).await
    }

    pub async fn put_records(&self, records: Vec<Record>) -> Result<RecordPutResp> {
        let res = self
            .client
            .put(format!("{}/records", self.base_url))
            .json(&records)
            .send()
            .await?;
        let results: RecordPutResp = res.json().await?;
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client_from_env() -> APIClient {
        APIClient::from_file(".env").unwrap()
    }
}
