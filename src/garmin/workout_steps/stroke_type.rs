use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="snake_case")]
pub enum Stroke{
    AnyStroke,
    Free,
    Breast,
    Back,
    Butterfly,
    IndividualMedley
}


#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StrokeType {
    pub stroke_type_key: Option<Stroke>,
}


impl StrokeType {

    pub fn new(stroke_type_key: Option<Stroke>) -> Self {
        StrokeType{stroke_type_key}
    }
    pub fn stroke_type_id(&self) -> u8 {
        match self.stroke_type_key {
            Some(Stroke::AnyStroke) => 1,
            Some(Stroke::Breast) => 2,      // Todo: correct
            Some(Stroke::Back) => 3,        // Todo: correct
            Some(Stroke::Butterfly) => 4,   // Todo: correct
            Some(Stroke::Free) => 6,
            Some(Stroke::IndividualMedley) => 7,
            None => 0,
        }
    }

    pub fn display_order(&self) -> u8 {
        self.stroke_type_id()
    }
}

impl Serialize for StrokeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("StrokeType", 3)?;
        state.serialize_field("strokeTypeId", &self.stroke_type_id())?;
        state.serialize_field("strokeTypeKey", &self.stroke_type_key)?;
        state.serialize_field("displayOrder", &self.display_order())?;
        state.end()
    }
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
        assert_eq!(json.stroke_type_id(), 6);
        assert_eq!(json.stroke_type_key, Some(Stroke::Free));
        assert_eq!(json.display_order(), 6);
    }

    #[test]
    fn test_serialize_stroke() {
        let stroke = StrokeType {
            stroke_type_key: Some(Stroke::IndividualMedley),
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
        assert_eq!(json.stroke_type_id(), 0);
        assert_eq!(json.stroke_type_key, None);
        assert_eq!(json.display_order(), 0);
    }

    #[test]
    fn test_serialize_null() {
        let resting_stroke = StrokeType {
            stroke_type_key: None,
        };
        let json_str = serde_json::to_string(&resting_stroke).unwrap();
        let expected = r#"{"strokeTypeId":0,"strokeTypeKey":null,"displayOrder":0}"#;
        assert_eq!(json_str, expected);
    }
}