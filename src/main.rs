use serde_json::Value;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let value: Value = serde_json::from_str(input.trim()).expect("Invalid JSON");

    let mut output = String::new();
    json_to_interfaces("Root", &value, &mut output);

    print!("{}", output.trim());
}

fn json_to_interfaces(name: &str, value: &Value, out: &mut String) {
    if let Value::Object(map) = value {
        let mut nested: Vec<(String, Value)> = vec![];

        out.push_str(&format!("interface {} {{\n", name));

        for (key, val) in map {
            let ts_type = infer_type(key, val, &mut nested);
            let optional = if val.is_null() { "?" } else { "" };
            out.push_str(&format!("  {}{}: {};\n", key, optional, ts_type));
        }

        out.push_str("}\n\n");

        for (nested_name, nested_val) in nested {
            json_to_interfaces(&nested_name, &nested_val, out);
        }
    }
}

fn infer_type(key: &str, value: &Value, nested: &mut Vec<(String, Value)>) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Object(_) => {
            let nested_name = capitalize(key);
            nested.push((nested_name.clone(), value.clone()));
            nested_name
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                return "unknown[]".to_string();
            }

            let mut types: Vec<String> = arr
                .iter()
                .map(|v| simple_type(v))
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            types.sort();

            if types.len() > 1 {
                return format!("({})[]", types.join(" | "));
            }

            let first = &arr[0];
            if first.is_object() {
                let nested_name = capitalize(key) + "Item";
                nested.push((nested_name.clone(), first.clone()));
                format!("{}[]", nested_name)
            } else {
                format!("{}[]", infer_type(key, first, nested))
            }
        }
    }
}

fn simple_type(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Object(_) => "object".to_string(),
        Value::Array(_) => "unknown[]".to_string(),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
