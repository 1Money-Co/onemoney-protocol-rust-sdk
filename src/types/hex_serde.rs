use serde::{Deserialize as _, Deserializer, Serializer};

use crate::io::{Export, Import, prefix_hex_string};

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Export<true>,
    S: Serializer,
{
    serializer.serialize_str(&prefix_hex_string(value).map_err(serde::ser::Error::custom)?)
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Import<true>,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::import(&mut s.as_bytes()).map_err(serde::de::Error::custom)
}
