use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepType {
    /*
    Todo:
        - Enum of type keys
        - Derive id and display_order based on key
     */
    step_type_id: u8,
    step_type_key: String,
    display_order: u8
}

#[cfg(test)]
mod tests {
    use crate::steps::StepType;

    #[test]
    fn test_deserialize_step() {
        let json_str = r#"
          {          
            "stepTypeId": 1,
            "stepTypeKey": "warmup",
            "displayOrder": 1
          }
        "#;
        let json: StepType = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.step_type_id, 1);
        assert_eq!(json.step_type_key, "warmup");
        assert_eq!(json.display_order, 1);
    }

    #[test]
    fn test_serialize_step() {
        let step = StepType {
            step_type_id: 2,
            step_type_key: "cooldown".to_string(),
            display_order: 2
        };
        let json_str = serde_json::to_string(&step).unwrap();
        let expected = r#"{"stepTypeId":2,"stepTypeKey":"cooldown","displayOrder":2}"#;
        assert_eq!(json_str, expected);
    }
}