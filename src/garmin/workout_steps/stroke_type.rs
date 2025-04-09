use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrokeType {
    /*
    Todo:
        - Enum of type keys
        - Derive id and display_order based on key
     */
    pub stroke_type_id: u8,
    pub stroke_type_key: Option<String>,
    pub display_order: u8
}

#[cfg(test)]
mod tests {

    use super::*;

#[test]
    fn test_deserialize_stroke() {
        let json_str = r#"
          {
            "strokeTypeId": 6,
            "strokeTypeKey": "free",
            "displayOrder": 6
          }
        "#;
        let json: StrokeType = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.stroke_type_id, 6);
        assert_eq!(json.stroke_type_key, Some("free".to_string()));
        assert_eq!(json.display_order, 6);
    }

    #[test]
    fn test_serialize_stroke() {
        let stroke = StrokeType {
            stroke_type_id: 7,
            stroke_type_key: Some("individual_medley".to_string()),
            display_order: 7
        };
        let json_str = serde_json::to_string(&stroke).unwrap();
        let expected = r#"{"strokeTypeId":7,"strokeTypeKey":"individual_medley","displayOrder":7}"#;
        assert_eq!(json_str, expected);
    }

    #[test]
    fn test_deserialize_null(){
        let json_str = r#"
         {
            "strokeTypeId": 0,
            "strokeTypeKey": null,
            "displayOrder": 0
          }
        "#;
        let json: StrokeType = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.stroke_type_id, 0);
        assert_eq!(json.stroke_type_key, None);
        assert_eq!(json.display_order, 0);
    }

    #[test]
    fn test_serialize_null() {
        let resting_stroke = StrokeType {
            stroke_type_id : 0,
            stroke_type_key: None,
            display_order: 0
        };
        let json_str = serde_json::to_string(&resting_stroke).unwrap();
        let expected = r#"{"strokeTypeId":0,"strokeTypeKey":null,"displayOrder":0}"#;
        assert_eq!(json_str, expected);
    }
}