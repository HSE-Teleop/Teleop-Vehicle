use std::string::ToString;
use std::sync::Arc;
use kuksa_rust_sdk::kuksa::common::ClientTraitV2;
use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;
use kuksa_rust_sdk::v2_proto;
use std::time::Duration;

mod utils;
use utils::utils::get_type_of;
use utils::kuksa_utils::create_kuksa_client;

use zenoh::{Config, Session};
use zenoh::bytes::ZBytes;
use crate::utils::zenoh_utils::create_zenoh_session;

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

async fn subscribe_to_vehicle_speed(vss_path: String) {
    println!("Subscribing to vehicle...");
    let mut v2_client = create_kuksa_client("").await;
    match v2_client.subscribe(vec![vss_path.to_owned()], None).await
    {
        Ok(mut stream) => {
            println!("✅ Subscribed to {}!", vss_path);
            tokio::spawn(async move {
                // KEEP READING until the server closes the stream or an error occurs
                let session = zenoh::open(zenoh::Config::default()).await.unwrap();
                while let Ok(Some(response)) = stream.message().await {
                    for (_path, datapoint) in response.entries {
                        println!("{}: {:?}", vss_path, datapoint);
                        session.put("Vehicle.ADAS.PowerOptimizeLevel", "1").await.unwrap();
                    }
                }
                session.close().await.unwrap();
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
    let mut v2_client = create_kuksa_client("").await;
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





async fn handle_databroker_publication(zenoh_session: Session, vss_paths: String) {
    let mut v2_client = create_kuksa_client("").await;
    let paths = vec![vss_paths.to_owned()];         // Can't subscribe to all paths in tree
    println!("✅ Subscribed to {:?}!", paths);
    match v2_client.subscribe(paths, None).await
    {
        Ok(mut stream) => {
            // let session = zenoh::open(zenoh::Config::default()).await.unwrap();
            while let Ok(Some(response)) = stream.message().await {
                for (_path, datapoint) in response.entries {
                    let value: u8 = 1;
                    // let value = datapoint.to_owned().value.unwrap().typed_value.unwrap();
                    let payload = ZBytes::from(&value.to_be_bytes()[..]);
                    println!("Published {:?} -> '{:?}' on path {}: {:?}", value, payload, _path, datapoint);
                    zenoh_session.put(_path.replace(".", "/"), payload).await.unwrap();
                }
            }
            // session.close().await.unwrap();
            println!("⚠️ Subscription to {} ended or errored out", vss_paths);
        }
        Err(err) => {
            println!("❌ Failed to subscribe to: {:?}", err);
        }
    }
}



#[tokio::main]
async fn main() {
    println!("Starting server...");
    // subscribe_to_vehicle_speed("Vehicle.ADAS.PowerOptimizeLevel".parse().unwrap()).await;
    // update_vehicle_speed("Vehicle.ADAS.PowerOptimizeLevel".to_owned(), 5).await;

    let zenoh_session = create_zenoh_session(CONFIG).await;
    
    let databroker_handle = tokio::spawn({
        // let session = Arc::clone(&zenoh_session);
        let session = zenoh_session.clone();
        handle_databroker_publication(zenoh_session, "Vehicle.ADAS.PowerOptimizeLevel".parse().unwrap())
    });

    let _ = databroker_handle.await;
    
    /*
    let session = create_zenoh_session(CONFIG).await;
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

    let subscriber_handle = tokio::spawn({
        let session = Arc::clone(&zenoh_session);
        let metadata_store = Arc::clone(&metadata_store);
        handling_zenoh_subscription(session, metadata_store, client)
    });

    let publisher_handle = tokio::spawn({
        let session = Arc::clone(&zenoh_session);
        publish_to_zenoh(provider_config, session, actuation_client)
    });

    let _ = subscriber_handle.await;
    let _ = publisher_handle.await;
    Ok(())*/
}