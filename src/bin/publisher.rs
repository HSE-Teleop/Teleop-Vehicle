use std::any::Any;
use std::hash::RandomState;
use std::time::Duration;
use rand::prelude::*;

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

    let mut value: u8 = 254;
    
    println!("Press CTRL-C to quit...");
    loop {
        // Refer to z_bytes.rs to see how to serialize different types of messages
        let payload = ZBytes::from(&value.to_be_bytes()[..]);
        println!("Putting Data ('Vehicle/ADAS/PowerOptimizeLevel': '{}' | {:?})...",
                 value,
                 payload
        );
        publisher
            .put(payload)
            .await
            .unwrap();

        // u8 goes from 0-255
        // Here 10 because the subscribed key has only 10 levels
        if(value >= 10) {
            value = 2;      // Resets level to 2 to reserve 0 & 1 for databroker
        } else {
            // A little bit of randomness
            // let mut rng = rand::rng();
            value += (random::<u8>() % 2) as u8;
        }
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}