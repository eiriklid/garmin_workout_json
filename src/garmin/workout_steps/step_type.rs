use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Step {
    Warmup,
    Cooldown,
    Rest,
    Repeat,
    Main,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StepType {
    pub step_type_key: Step,
}

impl StepType {

    pub fn step_type_id(&self) -> u8 {
        match self.step_type_key {
            Step::Warmup => 1,
            Step::Cooldown => 2,
            Step::Rest => 5,
            Step::Repeat => 6,
            Step::Main => 8,
        }
    }

    pub fn display_order(&self) -> u8 {
        self.step_type_id()
    }
}

impl Serialize for StepType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("StepType", 3)?;
        state.serialize_field("stepTypeId", &self.step_type_id())?;
        state.serialize_field("stepTypeKey", &self.step_type_key)?;
        state.serialize_field("displayOrder", &self.display_order())?;
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
            "stepTypeId": 1,
            "stepTypeKey": "warmup",
            "displayOrder": 1
          }
        "#;
        let json: StepType = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.step_type_id(), 1);
        assert_eq!(json.step_type_key, Step::Warmup);
        assert_eq!(json.display_order(), 1);
    }

    #[test]
    fn test_serialize_step() {
        let step = StepType {
            step_type_key: Step::Cooldown,
        };
        let json_str = serde_json::to_string(&step).unwrap();
        let expected = r#"{"stepTypeId":2,"stepTypeKey":"cooldown","displayOrder":2}"#;
        assert_eq!(json_str, expected);
    }
}