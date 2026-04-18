use serde_json::{Map, Value};
use std::collections::HashSet;
use std::hash::Hash;

pub(crate) fn strip_prefix(state: &Map<String, Value>, prefix: &str) -> Map<String, Value> {
    state
        .clone()
        .iter()
        .filter_map(|(k, v)| {
            if let Some(k) = k.strip_prefix(prefix) {
                Some((k.to_string(), v.clone()))
            } else {
                None
            }
        })
        .collect::<Map<String, Value>>()
}

pub(crate) fn insert_base(
    state: &Map<String, Value>,
    out: &mut Map<String, Value>,
    excluding: Option<Vec<&str>>,
) {
    state.iter().for_each(|(k, v)| {
        log::debug!("{}: {:?}", k, v);
        if !k.contains(".") {
            if excluding.is_none() || !excluding.as_ref().unwrap().contains(&k.as_str()) {
                out.insert(k.clone(), v.clone());
            }
        }
    });
}

pub(crate) fn trim_at(value: &Value, delimiter: &str) -> Value {
    Value::String(
        value
            .as_str()
            .unwrap()
            .split_once(delimiter)
            .unwrap()
            .0
            .to_string(),
    )
}

pub(crate) fn get_ids<T, F>(state: &Map<String, Value>, parse: F) -> HashSet<T>
where
    T: Hash + Eq,
    F: Fn(&str) -> T,
{
    log::debug!("get_ids {:?}", state);
    state
        .keys()
        .map(|k| parse(k.split_once(".").unwrap().0))
        .collect::<HashSet<T>>()
}
