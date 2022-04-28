use figment::map;
use itertools::Itertools;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use toml::Value as TomlValue;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    UInt(u64),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Value {
        match value {
            JsonValue::Null => Value::Null,
            JsonValue::Bool(ref b) => Value::Bool(*b),
            JsonValue::Number(n) => n
                .as_u64()
                .map(Value::UInt)
                .or_else(|| n.as_i64().map(Value::Int))
                .or_else(|| n.as_f64().map(Value::Float))
                .expect("Number must be int, uint or float, what's wrong?"),
            JsonValue::String(s) => Value::String(s),
            JsonValue::Array(a) => Value::Array(a.into_iter().map(Value::from).collect_vec()),
            JsonValue::Object(o) => {
                let map = o.into_iter().fold(HashMap::new(), |mut acc, (key, value)| {
                    acc.insert(key, Value::from(value));
                    acc
                });
                Value::Map(map)
            },
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum YamlConversionError {
    NonStringMapKey,
}

impl TryFrom<YamlValue> for Value {
    type Error = YamlConversionError;

    fn try_from(value: YamlValue) -> Result<Value, Self::Error> {
        let res = match value {
            YamlValue::Null => Value::Null,
            YamlValue::Bool(ref b) => Value::Bool(*b),
            YamlValue::Number(n) => n
                .as_u64()
                .map(Value::UInt)
                .or_else(|| n.as_i64().map(Value::Int))
                .or_else(|| n.as_f64().map(Value::Float))
                .expect("Number must be int, uint or float, what's wrong?"),
            YamlValue::String(s) => Value::String(s),
            YamlValue::Sequence(a) => {
                let array = a
                    .into_iter()
                    .map(Ok)
                    .map(|x| Value::try_from(x?))
                    .collect::<Result<Vec<_>, _>>()?;
                Value::Array(array)
            },
            YamlValue::Mapping(m) => {
                let mut map = HashMap::with_capacity(m.capacity());
                for (key, value) in m.into_iter() {
                    map.insert(
                        key.as_str().ok_or(YamlConversionError::NonStringMapKey)?.to_string(),
                        Value::try_from(value)?,
                    );
                }
                Value::Map(map)
            },
        };
        Ok(res)
    }
}

impl From<TomlValue> for Value {
    fn from(value: TomlValue) -> Value {
        match value {
            TomlValue::String(s) => Value::String(s),
            TomlValue::Integer(i) => Value::Int(i),
            TomlValue::Float(f) => Value::Float(f),
            TomlValue::Boolean(ref b) => Value::Bool(*b),
            TomlValue::Datetime(d) => Value::String(d.to_string()),
            TomlValue::Array(a) => Value::Array(a.into_iter().map(Value::from).collect_vec()),
            TomlValue::Table(t) => {
                let map = t.into_iter().fold(HashMap::new(), |mut acc, (key, value)| {
                    acc.insert(key, Value::from(value));
                    acc
                });
                Value::Map(map)
            },
        }
    }
}
