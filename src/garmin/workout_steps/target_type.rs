use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all= "snake_case")]
pub enum Target{
    #[serde(rename = "no.target")]
    NoTarget,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TargetType {
    workout_target_type_key: Target,
}

impl TargetType {

    pub fn workout_target_type_id(&self) -> u8{
        match self.workout_target_type_key {
            Target::NoTarget => 1,
        }
    }

    pub fn display_order(&self) -> u8{
        self.workout_target_type_id()
    }
}
impl Default for TargetType {
    fn default() -> Self {
        TargetType {
            workout_target_type_key: Target::NoTarget,
        }
    }
}

impl Serialize for TargetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TargetType", 3)?;
        state.serialize_field("workoutTargetTypeId", &self.workout_target_type_id())?;
        state.serialize_field("workoutTargetTypeKey", &self.workout_target_type_key)?;
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
            "workoutTargetTypeId": 1,
            "workoutTargetTypeKey": "no.target",
            "displayOrder": 1
          }
        "#;
        let json: TargetType = serde_json::from_str(json_str).unwrap();
        assert_eq!(json.workout_target_type_id(), 1);
        assert_eq!(json.workout_target_type_key, Target::NoTarget);
        assert_eq!(json.display_order(), 1);
    }

    #[test]
    fn test_serialize_step() {
        let step = TargetType {
            workout_target_type_key: Target::NoTarget,
        };
        let json_str = serde_json::to_string(&step).unwrap();
        let expected = r#"{"workoutTargetTypeId":1,"workoutTargetTypeKey":"no.target","displayOrder":1}"#;
        assert_eq!(json_str, expected);
    }
}