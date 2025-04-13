use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit{
    unit_id: Option<u8>,
    unit_key: Option<String>,
    factor: Option<u8>
}