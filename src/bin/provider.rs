use std::time::Duration;
use kuksa_rust_sdk::kuksa::common::ClientTraitV2;
use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;
use kuksa_rust_sdk::v2_proto;

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

async fn create_sdv_client() -> KuksaClientV2 {
    println!("Creating client...");
    let host = "http://databroker:55555";
    KuksaClientV2::from_host(host)
}

async fn subscribe_to_vehicle_speed(vss_path: String) {
    println!("Subscribing to vehicle...");
    let mut v2_client = create_sdv_client().await;
    match v2_client.subscribe(vec![vss_path.to_owned()], None).await
    {
        Ok(mut stream) => {
            println!("✅ Subscribed to {}!", vss_path);
            tokio::spawn(async move {
                // KEEP READING until the server closes the stream or an error occurs
                while let Ok(Some(response)) = stream.message().await {
                    for (_path, datapoint) in response.entries {
                        println!("{}: {:?}", vss_path, datapoint);
                    }
                }
                println!("⚠️ Subscription to {} ended or errored out", vss_path);
            });
        }
        Err(err) => {
            println!("❌ Failed to subscribe to: {:?}", err);
        }
    }
}

async fn update_vehicle_speed(vss_path: String, value: u8) {
    println!("Updating vehicle speed...");
    let mut v2_client = create_sdv_client().await;
    match v2_client.publish_value(
        vss_path.to_owned(),
        v2_proto::Value {
            typed_value: Some(v2_proto::value::TypedValue::Uint32(value.into())),
        },
    )
        .await
    {
        Ok(_) => {
            println!(
                "Value published successful for signal {:?}",
                vss_path
            );
        }
        Err(err) => {
            println!(
                "Publishing value for signal {:?} failed: {:?}",
                vss_path, err
            );
        }
    }
}

fn get_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[tokio::main]
async fn main() {
    println!("Starting server...");
    subscribe_to_vehicle_speed("Vehicle.ADAS.PowerOptimizeLevel".parse().unwrap()).await;
    update_vehicle_speed("Vehicle.ADAS.PowerOptimizeLevel".to_owned(), 5).await;

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
    // 4) Loop forever, printing each incoming value
    println!("Listening for updates every second. Press CTRL-C to quit.");
    
    let mut recived_power_optimize_level: u8 = 5;
    while let Ok(sample) = subscriber.recv_async().await {
        println!(
            "← [{}] '{}' → {:?}",
            sample.kind(),
            sample.key_expr().as_str(),
            sample
        );

        let z_key_value = sample.payload();
        let vector_value = z_key_value.slices().fold(Vec::new(), |mut b, x| { b.extend_from_slice(x); b });
        let key_value = vector_value[0];
        
        println!("Value: {:?} : {} (Bytes: {}), Type: '{}' -> '{}' : '{}'",
                 vector_value,
                 key_value,
                 z_key_value.len(),
                 get_type_of(z_key_value),
                 get_type_of(&vector_value),
                 get_type_of(&key_value)
        );
        
        if(recived_power_optimize_level != key_value) {
            recived_power_optimize_level = key_value;
            update_vehicle_speed("Vehicle.ADAS.PowerOptimizeLevel".to_owned(), recived_power_optimize_level).await;
        } else {
            println!("Value {} not changed", key_value);
        }
        
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}