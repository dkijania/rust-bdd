use serde::Serialize;

pub fn get_value<T: Serialize>(obj: T, name: &str) -> String {
    let text = serde_json::to_string(&obj).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    let object = value.as_object().unwrap();
    format!("{:?}", object[name])
}
