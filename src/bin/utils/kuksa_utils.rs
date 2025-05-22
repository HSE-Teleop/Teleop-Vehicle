use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;

pub async fn create_kuksa_client(host_url: &'static str) -> KuksaClientV2 {
    println!("Creating client...");
    let host = if host_url.is_empty() {
        "http://databroker:55555"
    } else {
        host_url
    };
    KuksaClientV2::from_host(host)
}