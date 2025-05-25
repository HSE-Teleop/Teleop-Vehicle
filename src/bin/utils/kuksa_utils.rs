use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;
use kuksa_rust_sdk::v2_proto::value::TypedValue;
use zenoh::bytes::ZBytes;
use crate::utils::utils::unwrap_typed_value;

pub async fn create_kuksa_client(host_url: &'static str) -> KuksaClientV2 {
    println!("Creating client...");
    let host = if host_url.is_empty() {
        "http://databroker:55555"
    } else {
        host_url
    };
    KuksaClientV2::from_host(host)
}

/// Converts a TypedValue into zenoh bytes by downcasting and parsing it to a string
pub fn s_typed_value_to_zenoh_bytes(value: TypedValue) -> ZBytes {
    let mut zenoh_bytes = ZBytes::from("None");         // Default value
    if let Some(boxed_value) = unwrap_typed_value(value.clone()) {
        zenoh_bytes = match boxed_value.downcast::<String>() {
            Ok(cast_value) => {
                // ZBytes::from(&cast_value.to_string()[..])
                ZBytes::from(&cast_value.to_string())
            }
            Err(boxed_value) => match boxed_value.downcast::<u8>() {
                Ok(cast_value) => {
                    ZBytes::from(&cast_value.to_string())
                }
                Err(boxed_value) => match boxed_value.downcast::<bool>() {
                    Ok(cast_value) => {
                        ZBytes::from(&cast_value.to_string())
                    }
                    Err(boxed_value) => match boxed_value.downcast::<u32>() {
                        Ok(cast_value) => {
                            ZBytes::from(&cast_value.to_string())
                        }
                        Err(_) => {
                            // Handle other types
                            eprintln!("Error: Type {:?} not implemented!", value);
                            ZBytes::from("None")            // Default value
                        }
                    }
                }
            }
        };
    }
    zenoh_bytes
}

/// Converts a TypedValue into zenoh bytes by downcasting to the right type
/// <br/>
/// Help: Is not uniform
pub fn typed_value_to_zenoh_bytes(value: TypedValue) -> ZBytes {
    let mut zenoh_bytes = Default::default();
    if let Some(boxed_value) = unwrap_typed_value(value.clone()) {
        zenoh_bytes = match boxed_value.downcast::<String>() {
            Ok(cast_value) => {
                ZBytes::from(&cast_value.to_string())
            }
            Err(boxed_value) => match boxed_value.downcast::<u8>() {
                Ok(cast_value) => {
                    ZBytes::from(&cast_value.to_be_bytes()[..])
                }
                Err(boxed_value) => match boxed_value.downcast::<bool>() {
                    Ok(cast_value) => {
                        ZBytes::from(&cast_value.to_string())
                    }
                    Err(boxed_value) => match boxed_value.downcast::<u32>() {
                        Ok(cast_value) => {
                            ZBytes::from(&cast_value.to_be_bytes()[..])
                        }
                        Err(_) => {
                            // Handle other types
                            eprintln!("Error: Type {:?} not implemented!", value);
                            ZBytes::from("-1")          // Default value
                        }
                    }
                }
            }
        };
    }
    zenoh_bytes
}