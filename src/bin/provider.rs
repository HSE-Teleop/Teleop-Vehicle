use kuksa_rust_sdk::kuksa::common::ClientTraitV2;
use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;
use kuksa_rust_sdk::v2_proto::value::TypedValue;
use kuksa_rust_sdk::v2_proto;
use std::collections::VecDeque;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod utils;
use utils::kuksa_utils::create_kuksa_client;

use crate::utils::kuksa_utils::{s_typed_value_to_zenoh_bytes, typed_value_to_string};
use crate::utils::provider_utils::{Message, MessageCache};
use crate::utils::utils::wrap_value_by_typed_value;
use crate::utils::zenoh_utils::create_zenoh_session;
use zenoh::bytes::ZBytes;
use zenoh::qos::Priority;
use zenoh::Session;

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
async fn handle_databroker_publication(message_cache: Arc<Mutex<MessageCache>>, mut v2_client: KuksaClientV2, zenoh_session: Session, signals: Vec<String>) {
    // let mut v2_client = create_kuksa_client("").await;
    // let paths = vec![vss_paths.to_owned()];         // Can't subscribe to all paths in the tree
    println!("✅ Subscribed to {:?}!", signals);
    match v2_client.subscribe(signals.clone(), None).await
    {
        Ok(mut stream) => {
            // let session = zenoh::open(zenoh::Config::default()).await.unwrap();
            while let Ok(Some(response)) = stream.message().await {
                for (_path, datapoint) in response.entries {
                    let value = datapoint.to_owned().value.unwrap_or_default().typed_value.unwrap_or(TypedValue::String("Empty".into()));
                    let payload = s_typed_value_to_zenoh_bytes(value.clone());
                    let parsed_message = typed_value_to_string(value.clone());
                    let signal = _path.replace(".", "/");

                    println!("DEBUG: {:?} => {} & '{:?}'", datapoint, parsed_message, payload);

                    let publish_to_zenoh: bool;
                    let double_message: Option<Message>;
                    // Comparing and discarding double messages
                    {
                        // Tries to acquire a lock
                        let mut mutex = message_cache.lock().unwrap();
                        (publish_to_zenoh, double_message) = mutex.expect_outgoing_message(
                            Message::new(parsed_message.clone(), signal.clone())
                        );
                    }

                    if publish_to_zenoh {
                        zenoh_session.put(
                            signal,
                            ZBytes::from(&parsed_message.clone()[..])
                        )
                            .priority(Priority::RealTime)
                            .await.unwrap();
                        println!("Published {:?} -> {}", value, _path);
                    } else {
                        println!("Debug: Found double {:?}", double_message.unwrap());
                    }
                }
                // tokio::time::sleep(Duration::from_secs(1)).await;
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
async fn handle_zenoh_communication(message_cache: Arc<Mutex<MessageCache>>, mut v2_client: KuksaClientV2, zenoh_session: Session, signals: Vec<String>, signal_types: Vec<String>) {
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
        // let received_bytes = sample.payload().slices().fold(Vec::new(), |mut b, x| { b.extend_from_slice(x); b });

        println!(
            "← [{}] '{}' → {}",
            sample.kind(),
            signal.clone(),
            msg
        );
        
        // Convert string into corresponding type
        let parsed_signal = signal.clone().replace("/", ".");
        let signal_index = signals.iter().position(|x| x.contains(parsed_signal.clone().as_str())).unwrap();
        
        let value = wrap_value_by_typed_value(msg.clone(), signal_types[signal_index].clone());
        let inferred_value = typed_value_to_string(value.clone());
        
        println!("DEBUG: {} -> {:?}\n{:?}", 
                 signal_types[signal_index].clone(), 
                 value,
                 sample);
        
        // Checking if the provider can resolve the correct value for the specific path
        // If the received string from the zenoh message differs from the inferred value, then stop the publishing process
        if msg != inferred_value {
            println!("Incoming message {} differs from inferred value {}", msg, inferred_value);
            println!("Maybe there was an invalid type on this path. Check your configuration!");
            continue;
        }

        // Checks if the zenoh message is tagged by the provider itself
        // Tagging is done by setting the priority of the message to RealTime
        if sample.priority() == Priority::RealTime {
            println!("Discarding incoming message\n");
            continue;
        }
        
        // Caching incoming messages
        {
            // Tries to acquire a lock
            let mut mutex = message_cache.lock().unwrap();
            mutex.push_message(inferred_value.clone(), signal.clone());
        }

        /**/
        // Publishing to the databroker
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
        }// */
        // tokio::time::sleep(Duration::from_secs(3)).await;
    }
}


#[tokio::main]
async fn main() {
    println!("Starting provider...");

    // Using own storage for the provider to handle messages
    let provider_queue = Arc::new(Mutex::new( MessageCache { message: VecDeque::new() } ));
    // Creating kuksa client to subscribe on the databroker
    let v2_client = create_kuksa_client("").await;
    // Creating another kuksa client for zenoh communication
    let v2_client_actuation = create_kuksa_client("").await;
    let zenoh_session = create_zenoh_session(CONFIG).await;
    // let paths = "Vehicle.ADAS.PowerOptimizeLevel".parse().unwrap();
    // Later replace the array with the config
    let paths = vec![
        "Vehicle.Speed".to_string(),
        "Vehicle.Teleop.SteeringAngle".to_string(),
        "Vehicle.Teleop.EnginePower".to_string(),
        "Vehicle.Teleop.ControlCounter".to_string(),
        "Vehicle.Teleop.ControlTimestampMs".to_string(),
    ];
    let signal_types = vec![
        "float".to_string(),
        "int16".to_string(),
        "float".to_string(),
        "uint8".to_string(),     // Should be uint8
        "uint32".to_string(),
    ];
    
    let databroker_handle = tokio::spawn({
        handle_databroker_publication(
            Arc::clone(&provider_queue),
            v2_client,
            zenoh_session.clone(),
            paths.clone()
        )
    });
    
    let zenoh_handle = tokio::spawn({
        handle_zenoh_communication(
            Arc::clone(&provider_queue),
            v2_client_actuation,
            zenoh_session.clone(),
            paths.clone(),
            signal_types.clone()
        )
    });
    
    let _ = databroker_handle.await;
    let _ = zenoh_handle.await;
}