use kuksa_rust_sdk::v2_proto::value::TypedValue;

/// Retrieves the type of the passed reference
pub fn get_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// Boxes the value of the received Datapoint into a trait which is allocated on the Heap and allows downcasting
pub fn unwrap_typed_value(value: TypedValue) -> Option<Box<dyn std::any::Any>> {
    match value {
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
pub fn wrap_value_by_typed_value(value: String, typed_value: String) -> TypedValue {
    match typed_value.as_str() {
        "string" => TypedValue::String(value.clone()),
        "bool" => TypedValue::Bool(value.parse::<bool>().unwrap_or(Default::default())),
        "int16" => TypedValue::Int32(value.parse::<i32>().unwrap_or(Default::default())),
        "uint8" => TypedValue::Uint32(value.parse::<u32>().unwrap_or(Default::default())),
        "uint32" => TypedValue::Uint32(value.parse::<u32>().unwrap_or(Default::default())),
        "float" => TypedValue::Float(value.parse::<f32>().unwrap_or(Default::default())),
        _ => { 
            panic!("Invalid type {}", typed_value);
            println!("Invalid type {}", typed_value);
            TypedValue::String("None".to_string())
        },
    }
}