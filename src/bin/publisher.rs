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

    println!("Declaring Publisher on 'Vehicle/Teleop/EnginePower'...");
    let publisher = session.declare_publisher("Vehicle/Teleop/EnginePower").await.unwrap();

    let mut value: f32 = 0.0;
    
    println!("Press CTRL-C to quit...");
    loop {
        if value >= 1.0 - f32::EPSILON {
            value = 0.0;
        }
        let shown = (value * 10.0).round() / 10.0;
        // Refer to z_bytes.rs to see how to serialize different types of messages
        let payload = ZBytes::from(&shown.to_be_bytes()[..]);
        let string_payload = ZBytes::from(format!("{:.1}", shown));
        println!("Putting Data ('Vehicle/Teleop/EnginePower': '{}' | {:?} / {:?})...",
                 shown,
                 payload,
                 string_payload
        );
        // publisher.put(payload).await.unwrap();
        publisher.put(string_payload).await.unwrap();
        
        value += 0.1;
        // Delays the zenoh message sometimes
        // if random::<i32>() % 2 == 0 {
            tokio::time::sleep(Duration::from_secs(2)).await;
        // }
    }
}