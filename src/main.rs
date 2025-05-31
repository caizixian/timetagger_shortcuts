use timetagger_shortcuts::*;

fn main() {
    let api_client = APIClient::client_from_file(".env");
    let records = api_client.get_running_records().unwrap();
    for r in records {
        println!("{:?}, tags: {:?}", r, r.tags());
    }
}
