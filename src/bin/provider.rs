use std::string::ToString;
use std::sync::Arc;
use kuksa_rust_sdk::kuksa::common::ClientTraitV2;
use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;
use kuksa_rust_sdk::{proto, v2_proto};
use std::time::Duration;
use clap::builder::TypedValueParser;
use kuksa_rust_sdk::v2_proto::value;
use kuksa_rust_sdk::v2_proto::value::TypedValue;

mod utils;
use utils::utils::get_type_of;
use utils::kuksa_utils::create_kuksa_client;

use zenoh::{Config, Session};
use zenoh::bytes::ZBytes;
use crate::utils::kuksa_utils::{s_typed_value_to_zenoh_bytes, typed_value_to_zenoh_bytes};
use crate::utils::utils::{unwrap_typed_value, wrap_value_by_typed_value};
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

/// Handles the publication of the databroker and publishes it to the zenoh network
/// 
/// Signals:
///     Downcasts the datapoint value to a string
async fn handle_databroker_publication(mut v2_client: KuksaClientV2, zenoh_session: Session, signals: Vec<String>) {
    // let mut v2_client = create_kuksa_client("").await;
    // let paths = vec![vss_paths.to_owned()];         // Can't subscribe to all paths in tree
    println!("✅ Subscribed to {:?}!", signals);
    match v2_client.subscribe(signals.clone(), None).await
    {
        Ok(mut stream) => {
            // let session = zenoh::open(zenoh::Config::default()).await.unwrap();
            while let Ok(Some(response)) = stream.message().await {
                for (_path, datapoint) in response.entries {
                    let value = datapoint.to_owned().value.unwrap_or_default().typed_value.unwrap_or(TypedValue::String("Empty".into()));
                    let payload = s_typed_value_to_zenoh_bytes(value.clone());
                    
                    // println!("DEBUG: {:?} => '{:?}'", datapoint, payload);
                    
                    zenoh_session.put(_path.replace(".", "/"), payload).await.unwrap();
                    println!("Published {:?} -> {}", value, _path);
                }
            }
            // session.close().await.unwrap();
            println!("⚠️ Subscription to {:?} ended or errored out", signals);
        }
        Err(err) => {
            println!("❌ Failed to subscribe to: {:?}", err);
        }
    }
}

/// Handles incoming traffic of the zenoh network and publishes it on the databroker
///
/// Signals:
///      Awaits signals encoded as strings
/// 
/// Conversion:
///     Uses the defined types to cast the signal value based on the path
async fn handle_zenoh_communication(mut v2_client: KuksaClientV2, zenoh_session: Session, signals: Vec<String>, signal_types: Vec<String>) {
    if signals.len() != signal_types.len() {
        panic!("Signals and signal types must be of the same length!");
    }
    
    println!("Declaring Subscriber on '{:?}'...", signals.clone());
    // Subscribes to all paths
    let subscriber = zenoh_session.declare_subscriber("Vehicle/**").await.unwrap();

    while let Ok(sample) = subscriber.recv_async().await {
        let signal = sample.key_expr().to_string();
        
        // Should convert everything into a string
        let msg = sample.payload().slices().map(|_byte| String::from_utf8(Vec::from(_byte)).unwrap()).fold(String::new(), |mut _msg, _char| { _msg.insert_str(_msg.len(), &*_char); _msg });
        // Received bytes in a vector
        let received_bytes = sample.payload().slices().fold(Vec::new(), |mut b, x| { b.extend_from_slice(x); b });

        println!(
            "← [{}] '{}' → {}",
            sample.kind(),
            signal.clone(),
            msg
        );
        
        // Convert string into corresponding type
        let parsed_signal = signal.clone().replace("/", ".");
        let signal_index = signals.iter().position(|x| x.contains(parsed_signal.clone().as_str())).unwrap();
        
        let value = wrap_value_by_typed_value(msg, signal_types[signal_index].clone());
        
        println!("DEBUG: {} -> {:?}", signal_types[signal_index].clone(), value);
        
        // Publish to databroker
        match v2_client.publish_value(
            parsed_signal.to_owned(),
            v2_proto::Value {
                typed_value: Some(value),
            },
        ).await {
            Ok(_) => {
                println!(
                    "Value published successful for signal {:?}",
                    parsed_signal
                );
            }
            Err(err) => {
                println!(
                    "Publishing value for signal {:?} failed: {:?}",
                    parsed_signal, err
                );
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}


#[tokio::main]
async fn main() {
    println!("Starting provider...");

    let v2_client = create_kuksa_client("").await;
    // Creating another kuksa client for zenoh communication
    let v2_client_actuation = create_kuksa_client("").await;
    let zenoh_session = create_zenoh_session(CONFIG).await;
    // let paths = "Vehicle.ADAS.PowerOptimizeLevel".parse().unwrap();
    // Later replace the array with the config
    let paths = vec![
        "Vehicle.Body.Horn.IsActive".to_string(),
        "Vehicle.ADAS.PowerOptimizeLevel".to_string(),
        "Vehicle.ADAS.CruiseControl.AdaptiveDistanceSet".to_string(),
    ];
    let signal_types = vec![
        "bool".to_string(),
        "uint8".to_string(),
        "float".to_string(),
    ];
    
    let databroker_handle = tokio::spawn({
        handle_databroker_publication(v2_client, zenoh_session.clone(), paths.clone())
    });
    
    let zenoh_handle = tokio::spawn({
        handle_zenoh_communication(v2_client_actuation, zenoh_session, paths.clone(), signal_types.clone())
    });
    
    let _ = databroker_handle.await;
    let _ = zenoh_handle.await;
}