use serde_json::Value;

pub fn validate_schema(schema: &Value) -> Result<(), String> {
    let obj = schema.as_object().ok_or_else(|| "Schema must be an object".to_string())?;

    // reject empty
    if obj.is_empty() {
        return Err("Schema cannot be empty".to_string());
    }

    // oversized
    let s = schema.to_string();
    if s.len() > 16384 {
        return Err("Schema is oversized".to_string());
    }

    // reject non-object root type if 'type' is specified
    if let Some(t) = obj.get("type") {
        if t != "object" {
            return Err("Schema root type must be object".to_string());
        }
    } else {
        return Err("Schema must have type: object".to_string());
    }

    // uncompilable check? We don't have a jsonschema compiler, but we can do basics.
    Ok(())
}
