use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit{
    pub(crate) unit_id: Option<u8>,
    pub(crate) unit_key: Option<String>,
    pub(crate) factor: Option<f32>
}


impl Default for Unit {
    fn default() -> Self {
        Unit{
            unit_id: Some(1),
            unit_key: Some("meter".to_string()),
            factor: Some(100.0),
        }
    }

}