use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sport{
    Swimming
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SportType {
    sport_type_key: Sport
}

impl SportType {
    fn sport_type_id(&self) -> u8{
        match self.sport_type_key {
            Sport::Swimming => 4,
        }
    }

    fn display_order(&self) -> u8 {
        self.sport_type_id() - 1
    }

    pub fn new(sport: Sport) -> SportType {
        SportType{sport_type_key: sport}
    }
}

impl Serialize for SportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SportType", 3)?;
        state.serialize_field("sportTypeId", &self.sport_type_id())?;
        state.serialize_field("sportTypeKey", &self.sport_type_key)?;
        state.serialize_field("displayOrder", &self.display_order())?;
        state.end()
    }
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

        assert_eq!(result.sport_type_id(), 4);
        assert_eq!(result.sport_type_key, Sport::Swimming);
        assert_eq!(result.display_order(), 3);
    }

    #[test]
    fn test_serialize() {

        let expected_json = r#"{
            "sportTypeId": 4,
            "sportTypeKey": "swimming",
            "displayOrder": 3
         }
         "#.chars().filter(|c| !c.is_whitespace()).collect::<String>();

        let object = SportType{sport_type_key: Sport::Swimming};

        let json_result = serde_json::to_string(&object).unwrap();

        assert_eq!(json_result, expected_json);
    }
}