use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Unit{
    Meter
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferredEndConditionUnit {
    unit_key: Unit,
}

impl PreferredEndConditionUnit {

    pub fn unit_id(&self) -> u8{
        match self.unit_key {
            Unit::Meter => 1,
        }
    }

    pub fn factor(&self) -> f32{
        match self.unit_key {
            Unit::Meter => 100.0,
        }
    }
}



impl Default for PreferredEndConditionUnit {
    fn default() -> Self {
        PreferredEndConditionUnit{
            unit_key: Unit::Meter,
        }
    }
}


impl Serialize for PreferredEndConditionUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PreferredEndCondition", 3)?;
        state.serialize_field("unitId", &self.unit_id())?;
        state.serialize_field("unitKey", &self.unit_key)?;
        state.serialize_field("factor", &self.factor())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_step() {
        let json_str = r#"
          {
            "unitId": 1,
            "unitKey": "meter",
            "factor": 100.0
          }
        "#;
        let json: PreferredEndConditionUnit = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.unit_id(), 1);
        assert_eq!(json.unit_key, Unit::Meter);
        assert_eq!(json.factor(), 100.0);
    }

    #[test]
    fn test_serialize_step() {
        let step = PreferredEndConditionUnit {
            unit_key: Unit::Meter,
        };
        let json_str = serde_json::to_string(&step).unwrap();
        let expected = r#"{"unitId":1,"unitKey":"meter","factor":100.0}"#;
        assert_eq!(json_str, expected);
    }
}