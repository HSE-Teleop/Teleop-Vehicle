use std::time::Duration;

use clap::Parser;
use zenoh::{bytes::Encoding, key_expr::KeyExpr, Config};
use zenoh::bytes::ZBytes;

const CONFIG: &str = 
    r#"{
        "mode": "client",
        "connect": {
            "endpoints": ["tcp/zenoh:7447"],
            "timeout_ms": -1,
            "exit_on_failure": false
        }
    }"#;

#[tokio::main]
async fn main() {
    // Initiate logging
    zenoh::init_log_from_env_or("error");
    
    let config = Config::from_json5(CONFIG).unwrap();

    println!("Opening session...");
    let session = zenoh::open(config).await.unwrap();

    println!("Declaring Publisher on 'Vehicle/ADAS/PowerOptimizeLevel'...");
    let publisher = session.declare_publisher("Vehicle/ADAS/PowerOptimizeLevel").await.unwrap();

    let value: i32 = 5;
    
    println!("Press CTRL-C to quit...");
    loop {    
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Putting Data ('Vehicle/ADAS/PowerOptimizeLevel': '{}')...", value);
        // Refer to z_bytes.rs to see how to serialize different types of message
        let payload = ZBytes::from(&value.to_be_bytes()[..]);
        publisher
            .put(payload)
            .await
            .unwrap();
    }
}