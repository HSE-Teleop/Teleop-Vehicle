use std::sync::Arc;
use std::time::Duration;
use kuksa_rust_sdk::kuksa::common::ClientTraitV2;
use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;
use kuksa_rust_sdk::v2_proto;

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

async fn update_vehicle_speed(vss_path: String) {
    println!("Updating vehicle speed...");
    let mut v2_client = create_sdv_client().await;
    match v2_client.publish_value(
        vss_path.to_owned(),
        v2_proto::Value {
            typed_value: Some(v2_proto::value::TypedValue::Uint32(5)),
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

#[tokio::main]
async fn main() {
    println!("Starting server...");
    subscribe_to_vehicle_speed("Vehicle.ADAS.PowerOptimizeLevel".parse().unwrap()).await;
    update_vehicle_speed("Vehicle.ADAS.PowerOptimizeLevel".to_owned()).await;

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}