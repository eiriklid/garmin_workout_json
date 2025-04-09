use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetType {
    /*
    Todo:
        - Enum of type keys
        - Derive id and display_order based on key
     */
    workout_target_type_id: u8,
    workout_target_type_key: String,
    display_order: u8,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize_step() {
        let json_str = r#"
          {
            "workoutTargetTypeId": 1,
            "workoutTargetTypeKey": "no.target",
            "displayOrder": 1
          }
        "#;
        let json: TargetType = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.workout_target_type_id, 1);
        assert_eq!(json.workout_target_type_key, "no.target");
        assert_eq!(json.display_order, 1);
    }

    #[test]
    fn test_serialize_step() {
        let step = TargetType {
            workout_target_type_id: 1,
            workout_target_type_key: "no.target".to_string(),
            display_order: 1,
        };
        let json_str = serde_json::to_string(&step).unwrap();
        let expected = r#"{"workoutTargetTypeId":1,"workoutTargetTypeKey":"no.target","displayOrder":1}"#;
        assert_eq!(json_str, expected);
    }
}