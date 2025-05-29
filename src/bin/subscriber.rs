use std::ops::Add;
use zenoh::Config;

mod utils;
use utils::utils::get_type_of;

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

    // 3) Declare a subscriber on all paths
    let key = "Vehicle/**";
    // let key = "Vehicle/ADAS/PowerOptimizeLevel";
    println!("Declaring Subscriber on '{}'…", key);
    let subscriber = session.declare_subscriber(key).await.unwrap();

    // 4) Loop forever, printing each incoming i32
    println!("Listening for updates every second. Press CTRL-C to quit.");
    while let Ok(sample) = subscriber.recv_async().await {
        let z_key_value = sample.payload();
        let vector_value = z_key_value.slices().fold(Vec::new(), |mut b, x| { b.extend_from_slice(x); b });
        
        // Should convert everything into a string
        let string_value = z_key_value.clone().slices().map(|_byte| String::from_utf8(Vec::from(_byte)).unwrap()).fold(String::new(), |mut _msg, _char| { _msg.insert_str(_msg.len(), &*_char); _msg });
        
        println!(
            "DEBUG: {:?} \nVector<u8>: {:?}",
            sample,
            vector_value
        );
        
        println!(
            "← [{}] '{}' → {}\n",
            sample.kind(),
            sample.key_expr().as_str(),
            string_value,
        );
    }
}
