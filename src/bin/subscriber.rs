use std::convert::TryInto;
use tokio::time::{sleep, Duration};
use zenoh::{bytes::ZBytes, Config};

/// Inline Zenoh JSON5 config to run as a client against your router.
const CONFIG: &str = r#"
{
  "mode": "client",
  "connect": {
    "endpoints": ["tcp/zenoh:7447"],
    "timeout_ms": -1,
    "exit_on_failure": false
  }
}
"#;

#[tokio::main]
async fn main(){
    // 1) Init logging
    zenoh::init_log_from_env_or("error");

    // 2) Parse and open session
    let config = Config::from_json5(CONFIG).unwrap();
    println!("Opening Zenoh client session…");
    let session = zenoh::open(config).await.unwrap();

    // 3) Declare a subscriber on our fixed key
    let key = "Vehicle/ADAS/PowerOptimizeLevel";
    println!("Declaring Subscriber on '{}'…", key);
    let subscriber = session.declare_subscriber(key).await.unwrap();

    // 4) Loop forever, printing each incoming i32
    println!("Listening for updates every second. Press CTRL-C to quit.");
    while let Ok(sample) = subscriber.recv_async().await {
        println!(
            "← [{}] '{}' → '{:?}'",
            sample.kind(),
            sample.key_expr().as_str(),
            sample.payload()
        );
    }
}
