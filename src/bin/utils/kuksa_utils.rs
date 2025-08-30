use std::any::{Any, TypeId};
use kuksa_rust_sdk::kuksa::val::v1::KuksaClient;
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
pub async fn create_kuksa_client_v1(host_url: &'static str) -> KuksaClient {
    println!("Creating client v1...");
    let host = if host_url.is_empty() {
        "http://databroker:55555"
    } else {
        host_url
    };
    KuksaClient::from_host(host)
}

/// Converts a TypedValue into zenoh bytes by downcasting and parsing it into a string
pub fn s_typed_value_to_zenoh_bytes(value: TypedValue) -> ZBytes {
    let mut zenoh_bytes = ZBytes::from("None");         // Default value
    if let Some(boxed_value) = unwrap_typed_value(value.clone()) {
        zenoh_bytes = match boxed_value.downcast::<String>() {
            // Convert String
            Ok(cast_value) => {
                ZBytes::from(&cast_value.to_string())
            }
            Err(boxed_value) => match boxed_value.downcast::<bool>() {
                // Convert boolean
                Ok(cast_value) => {
                    ZBytes::from(&cast_value.to_string())
                }
                Err(boxed_value) => match boxed_value.downcast::<i8>() {
                    // Convert int8 (byte)
                    Ok(cast_value) => {
                        ZBytes::from(&cast_value.to_string())
                    }
                    Err(boxed_value) => match boxed_value.downcast::<i16>() {
                        // Convert int16 (short)
                        Ok(cast_value) => {
                            ZBytes::from(&cast_value.to_string())
                        }
                        Err(boxed_value) => match boxed_value.downcast::<i32>() {
                            // Convert i32 (normal integer)
                            Ok(cast_value) => {
                                ZBytes::from(&cast_value.to_string())
                            }
                            Err(boxed_value) => match boxed_value.downcast::<u8>() {
                                // Convert uint8 (byte)
                                Ok(cast_value) => {
                                    ZBytes::from(&cast_value.to_string())
                                }
                                Err(boxed_value) => match boxed_value.downcast::<u16>() {
                                    // Convert uint16 (unsigned short)
                                    Ok(cast_value) => {
                                        ZBytes::from(&cast_value.to_string())
                                    }
                                    Err(boxed_value) => match boxed_value.downcast::<u32>() {
                                        // Convert uint32 (unsigned normal integer)
                                        Ok(cast_value) => {
                                            ZBytes::from(&cast_value.to_string())
                                        }
                                        Err(boxed_value) => match boxed_value.downcast::<f32>() {
                                            // Convert float
                                            Ok(cast_value) => {
                                                ZBytes::from(&cast_value.to_string())
                                            }
                                            Err(boxed_value) => match boxed_value.downcast::<f64>() {
                                                // Convert double
                                                Ok(cast_value) => {
                                                    ZBytes::from(&cast_value.to_string())
                                                    // ZBytes::from(&format!("{:?}", cast_value))
                                                }
                                                Err(_) => {
                                                    // Handle other types
                                                    eprintln!("Error: Type {:?} not implemented for conversion from TypedValue into ZBytes!", value);
                                                    ZBytes::from("None")            // Default value
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
    zenoh_bytes
}

// pub fn to_zenoh_bytes<T: std::fmt::Display + ToString + std::fmt::Debug + Into<TypedValue>>(value: T) -> ZBytes {
//     let zenoh_bytes = if value.type_id() == TypeId::of::<TypedValue>() {
//         s_typed_value_to_zenoh_bytes(TypedValue::clone(&value.into().clone()))
//     } else if value.type_id() == TypeId::of::<String>() {
//         ZBytes::from(&value.to_string().clone()[..])
//     } else {
//         eprintln!("Error: Type {:?} not implemented for conversion from generic type into ZBytes!", value);
//         ZBytes::from("None")
//     };
//     zenoh_bytes
// }

/// Converts a TypedValue into a string by trying to downcast to the right type
pub fn typed_value_to_string(value: TypedValue) -> String {
    let mut converted_string = Default::default();
    if let Some(boxed_value) = unwrap_typed_value(value.clone()) {
        converted_string = match boxed_value.downcast::<String>() {
            // Convert String
            Ok(cast_value) => {
                cast_value.to_string()
            }
            Err(boxed_value) => match boxed_value.downcast::<bool>() {
                // Convert boolean
                Ok(cast_value) => {
                    cast_value.to_string()
                }
                Err(boxed_value) => match boxed_value.downcast::<i8>() {
                    // Convert int8 (byte)
                    Ok(cast_value) => {
                        cast_value.to_string()
                    }
                    Err(boxed_value) => match boxed_value.downcast::<i16>() {
                        // Convert int16 (short)
                        Ok(cast_value) => {
                            cast_value.to_string()
                        }
                        Err(boxed_value) => match boxed_value.downcast::<i32>() {
                            // Convert i32 (normal integer)
                            Ok(cast_value) => {
                                cast_value.to_string()
                            }
                            Err(boxed_value) => match boxed_value.downcast::<u8>() {
                                // Convert uint8 (byte)
                                Ok(cast_value) => {
                                    cast_value.to_string()
                                }
                                Err(boxed_value) => match boxed_value.downcast::<u16>() {
                                    // Convert uint16 (unsigned short)
                                    Ok(cast_value) => {
                                        cast_value.to_string()
                                    }
                                    Err(boxed_value) => match boxed_value.downcast::<u32>() {
                                        // Convert uint32 (unsigned normal integer)
                                        Ok(cast_value) => {
                                            cast_value.to_string()
                                        }
                                        Err(boxed_value) => match boxed_value.downcast::<f32>() {
                                            // Convert float
                                            Ok(cast_value) => {
                                                format!("{:?}", cast_value)
                                            }
                                            Err(boxed_value) => match boxed_value.downcast::<f64>() {
                                                // Convert double
                                                Ok(cast_value) => {
                                                    format!("{:?}", cast_value)
                                                    // cast_value.to_string()
                                                }
                                                Err(_) => {
                                                    // Handle other types
                                                    eprintln!("Error: Type {:?} not implemented for conversion from TypedValue into String!", value);
                                                    "None".to_string()          // Default value
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
    converted_string
}