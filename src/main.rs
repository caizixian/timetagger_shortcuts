use timetagger_shortcuts::*;

fn main() {
    let api_client = APIClient::from_file(".env").unwrap();
    let records = api_client.get_running_records().unwrap();
    for r in &records {
        println!("{:?}, tags: {:?}", r, r.tags());
    }
    let stop_running_records: Vec<Record> = records
        .into_iter()
        .filter(|r| r.is_running())
        .map(|r| r.stop())
        .collect();
    api_client.put_records(stop_running_records).unwrap();
}
