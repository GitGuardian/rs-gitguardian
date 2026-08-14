use serde_json::{Value, json};

pub fn patch(spec: &mut Value) {
    patch_security_schemes(spec);
    patch_nullable_enums(spec);
}

fn patch_security_schemes(spec: &mut Value) {
    let Some(schemes) = spec
        .pointer_mut("/components/securitySchemes")
        .and_then(Value::as_object_mut)
    else {
        return;
    };

    for scheme in schemes.values_mut() {
        if scheme.get("type").and_then(Value::as_str) != Some("http") {
            continue;
        }
        let description = scheme.get("description").cloned();
        let mut replacement = json!({
            "type": "apiKey",
            "in": "header",
            "name": "Authorization",
        });
        if let Some(description) = description {
            replacement["description"] = description;
        }
        *scheme = replacement;
    }
}

fn patch_nullable_enums(node: &mut Value) {
    match node {
        Value::Object(map) => {
            if map.get("nullable").and_then(Value::as_bool) == Some(true)
                && let Some(values) = map.get_mut("enum").and_then(Value::as_array_mut)
                && !values.iter().any(Value::is_null)
            {
                values.push(Value::Null);
            }
            for value in map.values_mut() {
                patch_nullable_enums(value);
            }
        }
        Value::Array(values) => values.iter_mut().for_each(patch_nullable_enums),
        _ => {}
    }
}
