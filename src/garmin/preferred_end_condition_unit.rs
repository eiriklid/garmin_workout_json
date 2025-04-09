use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferredEndConditionUnit {
    /*
    Todo:
        - Enum of type keys
        - Derive id and display_order based on key
     */
    unit_id: u8,
    unit_key: String,
    factor: f32
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
        assert_eq!(json.unit_id, 1);
        assert_eq!(json.unit_key, "meter");
        assert_eq!(json.factor, 100.0);
    }

    #[test]
    fn test_serialize_step() {
        let step = PreferredEndConditionUnit {
            unit_id: 1,
            unit_key: "meter".to_string(),
            factor: 100.0
        };
        let json_str = serde_json::to_string(&step).unwrap();
        let expected = r#"{"unitId":1,"unitKey":"meter","factor":100.0}"#;
        assert_eq!(json_str, expected);
    }
}