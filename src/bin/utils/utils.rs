use kuksa_rust_sdk::v1_proto::datapoint::Value;
use kuksa_rust_sdk::v2_proto::value::TypedValue;

/// Retrieves the type of the passed reference
pub fn get_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// Boxes the value of the received Datapoint into a trait which is allocated on the Heap and allows downcasting
pub fn unwrap_typed_value(value: TypedValue) -> Option<Box<dyn std::any::Any>> {
    match value {
        // Those are the only types that are supported by the SDK
        TypedValue::String(val) => Some(Box::new(val)),
        TypedValue::Bool(val) => Some(Box::new(val)),
        TypedValue::Int32(val) => Some(Box::new(val)),
        TypedValue::Int64(val) => Some(Box::new(val)),
        TypedValue::Uint32(val) => Some(Box::new(val)),
        TypedValue::Uint64(val) => Some(Box::new(val)),
        TypedValue::Float(val) => Some(Box::new(val)),
        TypedValue::Double(val) => Some(Box::new(val)),
        TypedValue::StringArray(val) => Some(Box::new(val.values)),
        TypedValue::BoolArray(val) => Some(Box::new(val.values)),
        TypedValue::Int32Array(val) => Some(Box::new(val.values)),
        TypedValue::Int64Array(val) => Some(Box::new(val.values)),
        TypedValue::Uint32Array(val) => Some(Box::new(val.values)),
        TypedValue::Uint64Array(val) => Some(Box::new(val.values)),
        TypedValue::FloatArray(val) => Some(Box::new(val.values)),
        TypedValue::DoubleArray(val) => Some(Box::new(val.values)),
    }
}

/// Tries to parse value into known TypedValue
/// 
/// Extend type conversion in 'kuksa_utils' too!
pub fn wrap_value_by_typed_value(value: String, typed_value: String) -> TypedValue {
    match typed_value.as_str() {
        "string" => TypedValue::String(value.clone()),
        "bool" => TypedValue::Bool(value.parse::<bool>().unwrap_or(Default::default())),
        "int8" | "int16" | "int32" => TypedValue::Int32(value.parse::<i32>().unwrap_or(Default::default())),
        "uint8" | "uint16" | "uint32" => TypedValue::Uint32(value.parse::<u32>().unwrap_or(Default::default())),
        "float" => TypedValue::Float(value.parse::<f32>().unwrap_or(Default::default())),
        "double" => TypedValue::Double(value.parse::<f64>().unwrap_or(Default::default())),
        _ => { 
            panic!("Invalid type {}", typed_value);
            println!("Invalid type {}", typed_value);
            TypedValue::String("None".to_string())
        },
    }
}

/// Tries to parse value into known datapoint::Value
///
/// Doesn't need to be extended because type conversion is done with v2!
pub fn wrap_value_by_datapoint_value(value: String, typed_value: String) -> Value {
    match typed_value.as_str() {
        "string" => Value::String(value.clone()),
        "bool" => Value::Bool(value.parse::<bool>().unwrap_or(Default::default())),
        "int8" | "int16" | "int32" => Value::Int32(value.parse::<i32>().unwrap_or(Default::default())),
        "uint8" | "uint16" | "uint32" => Value::Uint32(value.parse::<u32>().unwrap_or(Default::default())),
        "float" => Value::Float(value.parse::<f32>().unwrap_or(Default::default())),
        "double" => Value::Double(value.parse::<f64>().unwrap_or(Default::default())),
        _ => {
            panic!("Invalid type {}", typed_value);
        },
    }
}