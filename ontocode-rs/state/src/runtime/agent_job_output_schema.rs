use anyhow::bail;
use serde_json::Map;
use serde_json::Value;

pub(super) fn validate_result_against_output_schema(
    schema: &Value,
    result: &Value,
) -> anyhow::Result<()> {
    validate_schema_node(schema, result, "$")
}

fn validate_schema_node(schema: &Value, value: &Value, path: &str) -> anyhow::Result<()> {
    let schema_object = schema
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("output_schema at {path} must be a JSON object"))?;
    validate_supported_keywords(schema_object, path)?;
    validate_combiner(schema_object, "allOf", value, path, Combiner::AllOf)?;
    validate_combiner(schema_object, "anyOf", value, path, Combiner::AnyOf)?;
    validate_combiner(schema_object, "oneOf", value, path, Combiner::OneOf)?;
    validate_const_and_enum(schema_object, value, path)?;
    validate_type(schema_object, value, path)?;
    validate_string_limits(schema_object, value, path)?;
    validate_numeric_limits(schema_object, value, path)?;
    validate_array_rules(schema_object, value, path)?;
    validate_object_rules(schema_object, value, path)?;
    Ok(())
}

fn validate_supported_keywords(
    schema_object: &Map<String, Value>,
    path: &str,
) -> anyhow::Result<()> {
    for key in schema_object.keys() {
        if matches!(
            key.as_str(),
            "$defs"
                | "$id"
                | "$schema"
                | "additionalProperties"
                | "allOf"
                | "anyOf"
                | "const"
                | "definitions"
                | "description"
                | "enum"
                | "format"
                | "items"
                | "maxItems"
                | "maxLength"
                | "maximum"
                | "minItems"
                | "minLength"
                | "minimum"
                | "oneOf"
                | "properties"
                | "required"
                | "title"
                | "type"
        ) {
            continue;
        }
        bail!("unsupported output_schema keyword `{key}` at {path}");
    }
    Ok(())
}

fn validate_const_and_enum(
    schema_object: &Map<String, Value>,
    value: &Value,
    path: &str,
) -> anyhow::Result<()> {
    if let Some(constraint) = schema_object.get("const")
        && constraint != value
    {
        bail!("output_schema validation failed at {path}: value did not match const");
    }

    if let Some(options) = schema_object.get("enum") {
        let options = options
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("output_schema enum at {path} must be an array"))?;
        if !options.iter().any(|candidate| candidate == value) {
            bail!("output_schema validation failed at {path}: value did not match enum");
        }
    }

    Ok(())
}

fn validate_type(
    schema_object: &Map<String, Value>,
    value: &Value,
    path: &str,
) -> anyhow::Result<()> {
    let Some(expected_type) = schema_object.get("type") else {
        return Ok(());
    };

    let matches = match expected_type {
        Value::String(kind) => value_matches_type(kind, value)?,
        Value::Array(kinds) => kinds.iter().try_fold(false, |matched, kind| {
            if matched {
                return Ok(true);
            }
            let kind = kind.as_str().ok_or_else(|| {
                anyhow::anyhow!("output_schema type array at {path} must contain strings")
            })?;
            value_matches_type(kind, value)
        })?,
        _ => bail!("output_schema type at {path} must be a string or array"),
    };

    if !matches {
        bail!("output_schema validation failed at {path}: type mismatch");
    }
    Ok(())
}

fn value_matches_type(kind: &str, value: &Value) -> anyhow::Result<bool> {
    Ok(match kind {
        "array" => value.is_array(),
        "boolean" => value.is_boolean(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "null" => value.is_null(),
        "number" => value.is_number(),
        "object" => value.is_object(),
        "string" => value.is_string(),
        _ => bail!("unsupported output_schema type `{kind}`"),
    })
}

fn validate_string_limits(
    schema_object: &Map<String, Value>,
    value: &Value,
    path: &str,
) -> anyhow::Result<()> {
    let Some(text) = value.as_str() else {
        return Ok(());
    };
    let length = text.chars().count();

    if let Some(min_length) = schema_object.get("minLength").and_then(Value::as_u64)
        && length < min_length as usize
    {
        bail!("output_schema validation failed at {path}: string shorter than minLength");
    }
    if let Some(max_length) = schema_object.get("maxLength").and_then(Value::as_u64)
        && length > max_length as usize
    {
        bail!("output_schema validation failed at {path}: string longer than maxLength");
    }
    Ok(())
}

fn validate_numeric_limits(
    schema_object: &Map<String, Value>,
    value: &Value,
    path: &str,
) -> anyhow::Result<()> {
    let Some(number) = value.as_f64() else {
        return Ok(());
    };

    if let Some(minimum) = schema_object.get("minimum").and_then(Value::as_f64)
        && number < minimum
    {
        bail!("output_schema validation failed at {path}: number below minimum");
    }
    if let Some(maximum) = schema_object.get("maximum").and_then(Value::as_f64)
        && number > maximum
    {
        bail!("output_schema validation failed at {path}: number above maximum");
    }
    Ok(())
}

fn validate_array_rules(
    schema_object: &Map<String, Value>,
    value: &Value,
    path: &str,
) -> anyhow::Result<()> {
    let Some(items) = value.as_array() else {
        return Ok(());
    };

    if let Some(min_items) = schema_object.get("minItems").and_then(Value::as_u64)
        && items.len() < min_items as usize
    {
        bail!("output_schema validation failed at {path}: array shorter than minItems");
    }
    if let Some(max_items) = schema_object.get("maxItems").and_then(Value::as_u64)
        && items.len() > max_items as usize
    {
        bail!("output_schema validation failed at {path}: array longer than maxItems");
    }

    let Some(item_schema) = schema_object.get("items") else {
        return Ok(());
    };
    match item_schema {
        Value::Object(_) => {
            for (index, item) in items.iter().enumerate() {
                validate_schema_node(item_schema, item, &format!("{path}[{index}]"))?;
            }
        }
        Value::Array(item_schemas) => {
            for (index, item) in items.iter().enumerate() {
                let Some(item_schema) = item_schemas.get(index) else {
                    bail!(
                        "output_schema validation failed at {path}: tuple item {index} missing schema"
                    );
                };
                validate_schema_node(item_schema, item, &format!("{path}[{index}]"))?;
            }
        }
        _ => bail!("output_schema items at {path} must be an object or array"),
    }
    Ok(())
}

fn validate_object_rules(
    schema_object: &Map<String, Value>,
    value: &Value,
    path: &str,
) -> anyhow::Result<()> {
    let Some(object) = value.as_object() else {
        return Ok(());
    };

    if let Some(required) = schema_object.get("required") {
        let required = required
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("output_schema required at {path} must be an array"))?;
        for key in required {
            let key = key.as_str().ok_or_else(|| {
                anyhow::anyhow!("output_schema required at {path} must contain strings")
            })?;
            if !object.contains_key(key) {
                bail!(
                    "output_schema validation failed at {path}: missing required property `{key}`"
                );
            }
        }
    }

    let properties = match schema_object.get("properties") {
        Some(Value::Object(properties)) => Some(properties),
        Some(_) => bail!("output_schema properties at {path} must be an object"),
        None => None,
    };

    if let Some(properties) = properties {
        for (key, child_value) in object {
            if let Some(child_schema) = properties.get(key) {
                validate_schema_node(child_schema, child_value, &format!("{path}.{key}"))?;
            }
        }
    }

    let Some(additional_properties) = schema_object.get("additionalProperties") else {
        return Ok(());
    };
    match additional_properties {
        Value::Bool(true) => Ok(()),
        Value::Bool(false) => {
            let Some(properties) = properties else {
                if object.is_empty() {
                    return Ok(());
                }
                bail!(
                    "output_schema validation failed at {path}: additional properties are not allowed"
                );
            };
            for key in object.keys() {
                if !properties.contains_key(key) {
                    bail!("output_schema validation failed at {path}: unexpected property `{key}`");
                }
            }
            Ok(())
        }
        Value::Object(_) => {
            for (key, child_value) in object {
                if properties.is_some_and(|known| known.contains_key(key)) {
                    continue;
                }
                validate_schema_node(additional_properties, child_value, &format!("{path}.{key}"))?;
            }
            Ok(())
        }
        _ => bail!("output_schema additionalProperties at {path} must be a bool or object"),
    }
}

#[derive(Clone, Copy)]
enum Combiner {
    AllOf,
    AnyOf,
    OneOf,
}

fn validate_combiner(
    schema_object: &Map<String, Value>,
    key: &str,
    value: &Value,
    path: &str,
    combiner: Combiner,
) -> anyhow::Result<()> {
    let Some(schemas) = schema_object.get(key) else {
        return Ok(());
    };
    let schemas = schemas
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("output_schema {key} at {path} must be an array"))?;

    match combiner {
        Combiner::AllOf => {
            for (index, schema) in schemas.iter().enumerate() {
                validate_schema_node(schema, value, &format!("{path}.{key}[{index}]"))?;
            }
        }
        Combiner::AnyOf => {
            if !schemas
                .iter()
                .any(|schema| validate_schema_node(schema, value, path).is_ok())
            {
                bail!("output_schema validation failed at {path}: did not satisfy anyOf");
            }
        }
        Combiner::OneOf => {
            let matched = schemas
                .iter()
                .filter(|schema| validate_schema_node(schema, value, path).is_ok())
                .count();
            if matched != 1 {
                bail!("output_schema validation failed at {path}: did not satisfy oneOf");
            }
        }
    }
    Ok(())
}
