
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndCondition {
    /*
    Todo:
        - Enum of type keys
        - Derive id and display_order based on key
     */
    condition_type_id: u8,
    condition_type_key: String,
    display_order: u8,
    displayable: bool,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize_step() {
        let json_str = r#"
          {
            "conditionTypeId": 3,
            "conditionTypeKey": "distance",
            "displayOrder": 3,
            "displayable": true
          }
        "#;
        let json: EndCondition = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.condition_type_id, 3);
        assert_eq!(json.condition_type_key, "distance");
        assert_eq!(json.display_order, 3);
        assert_eq!(json.displayable, true);
    }

    #[test]
    fn test_serialize_step() {
        let step = EndCondition {
            condition_type_id: 1,
            condition_type_key: "lap.button".to_string(),
            display_order: 1,
            displayable: true,
        };
        let json_str = serde_json::to_string(&step).unwrap();
        let expected = r#"{"conditionTypeId":1,"conditionTypeKey":"lap.button","displayOrder":1,"displayable":true}"#;
        assert_eq!(json_str, expected);
    }
}