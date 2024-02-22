use fsrs::MemoryState;
use rusqlite::types::{FromSql, ValueRef};

use rusqlite::ToSql;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_json::Value;

pub(crate) fn default_on_invalid<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CardData {
    #[serde(
        rename = "pos",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "default_on_invalid"
    )]
    pub(crate) original_position: Option<u32>,

    #[serde(
        rename = "s",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "default_on_invalid"
    )]
    pub(crate) fsrs_stability: Option<f32>,

    #[serde(
        rename = "d",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "default_on_invalid"
    )]
    pub(crate) fsrs_difficulty: Option<f32>,

    #[serde(
        rename = "dr",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "default_on_invalid"
    )]
    pub(crate) fsrs_desired_retention: Option<f32>,

    #[serde(default, rename = "cd", skip_serializing_if = "meta_is_empty")]
    pub(crate) custom_data: String,
}

fn meta_is_empty(s: &str) -> bool {
    matches!(s, "" | "{}")
}

impl FromSql for CardData {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        if let ValueRef::Text(s) = value {
            Ok(serde_json::from_slice(s).unwrap_or_default())
        } else {
            Ok(Self::default())
        }
    }
}

impl ToSql for CardData {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.custom_data.to_sql()
    }
}

impl CardData {
    pub fn memory_state(&self) -> Option<MemoryState> {
        if let Some(stability) = self.fsrs_stability {
            return Some(MemoryState {
                stability: stability,
                difficulty: self.fsrs_difficulty.unwrap_or_default(),
            });
        }
        None
    }
}
