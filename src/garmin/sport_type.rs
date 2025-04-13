use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SportType {
    sport_type_id: u8,
    sport_type_key: String,
    display_order: u8, // Note that this is not equal to sport_type_id in this case
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
            "sportTypeId": 4,
            "sportTypeKey": "swimming",
            "displayOrder": 3
          }
        "#;
        let result: SportType = serde_json::from_str(json).unwrap();

        assert_eq!(result.sport_type_id, 4);
        assert_eq!(result.sport_type_key, "swimming");
        assert_eq!(result.display_order, 3);
    }

    #[test]
    fn test_serialize() {

        let expected_json = r#"{
            "sportTypeId": 4,
            "sportTypeKey": "swimming",
            "displayOrder": 3
         }
         "#.chars().filter(|c| !c.is_whitespace()).collect::<String>();

        let object = SportType{ sport_type_id: 4, sport_type_key: "swimming".to_string(), display_order: 3 };

        let json_result = serde_json::to_string(&object).unwrap();

        assert_eq!(json_result, expected_json);
    }
}