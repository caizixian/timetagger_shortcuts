use timetagger_shortcuts::*;

fn main() {
    let api_client = APIClient::client_from_file(".env");
    println!("{:?}", api_client.get_running_records().unwrap());
}
