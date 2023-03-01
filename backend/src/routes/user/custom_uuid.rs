use std::str::FromStr;

use serde::{de::Error, Deserializer, Serialize, Serializer, Deserialize};
use uuid::Uuid;

pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    val.to_string().serialize(serializer)
}

pub fn _deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let val: &str = Deserialize::deserialize(deserializer)?;
    Uuid::from_str(val).map_err(D::Error::custom)
}
