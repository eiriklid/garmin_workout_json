use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Condition{
    // Todo: Find all cases
    #[serde(rename = "lap.button")]
    LapButton,
    Time,
    Distance,
    Iterations,
    #[serde(rename = "fixed.rest")]
    FixedRest,

}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EndCondition {
    pub condition_type_key: Condition,
    pub displayable: bool,
}


impl EndCondition {

    pub fn new(condition: Condition) -> Self {
        EndCondition{condition_type_key: condition, displayable: true}
    }
    pub fn condition_type_id(&self) -> u8 {
        match self.condition_type_key {
            Condition::LapButton => 1,
            Condition::Time => 2,
            Condition::Distance => 3,
            Condition::Iterations => 7,
            Condition::FixedRest => 8,
        }
    }

    pub fn display_order(&self) -> u8 {
        self.condition_type_id()
    }
}


impl Serialize for EndCondition{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("EndCondition", 4)?;
        state.serialize_field("conditionTypeId", &self.condition_type_id())?;
        state.serialize_field("conditionTypeKey", &self.condition_type_key)?;
        state.serialize_field("displayOrder", &self.display_order())?;
        state.serialize_field("displayable", &self.displayable)?;
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
            "conditionTypeId": 3,
            "conditionTypeKey": "distance",
            "displayOrder": 3,
            "displayable": true
          }
        "#;
        let json: EndCondition = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.condition_type_id(), 3);
        assert_eq!(json.condition_type_key, Condition::Distance);
        assert_eq!(json.display_order(), 3);
        assert_eq!(json.displayable, true);
    }

    #[test]
    fn test_serialize_step() {
        let step = EndCondition {
            condition_type_key: Condition::LapButton,
            displayable: true,
        };
        let json_str = serde_json::to_string(&step).unwrap();
        let expected = r#"{"conditionTypeId":1,"conditionTypeKey":"lap.button","displayOrder":1,"displayable":true}"#;
        assert_eq!(json_str, expected);
    }
}