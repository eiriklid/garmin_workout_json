use serde::{Deserialize, Serialize};
use crate::garmin::end_condition::EndCondition;
use crate::garmin::preferred_end_condition_unit::PreferredEndConditionUnit;
use crate::garmin::step_type::StepType;
use crate::garmin::stroke_type::StrokeType;
use crate::garmin::target_type::TargetType;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
struct ExecutableStepDTO{
    /*
    Todo:
        - Create convenience function to construct
        - Test serialize
     */

    step_id: u64,
    step_order: u8,
    step_type: StepType,
    child_step_id: Option<u8>,
    description: Option<String>,
    end_condition: EndCondition,
    end_condition_value: f32,
    preferred_end_condition_unit: PreferredEndConditionUnit,
    end_condition_compare: Option<bool>, // Unsure which type this field have
    target_type: TargetType,
    target_value_one: Option<f32>,
    target_value_two: Option<f32>,
    target_value_unit: Option<String>,
    zone_number: Option<u8>,
    secondary_target_type: Option<TargetType>,
    secondary_target_value_one: Option<f32>,
    secondary_target_value_two: Option<f32>,
    secondary_target_value_unit: Option<String>,
    secondary_zone_number: Option<u8>,
    end_condition_zone: Option<String>, // Unsure which type this field have
    stroke_type: StrokeType,
    equipment_type: EquipmentType,
    category: Option<String>,
    exercise_name: Option<String>,
    workout_provider: Option<String>,
    provider_exercise_source_id: Option<u32>,
    weight_value: Option<f32>,
    weight_unit: Option<String>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EquipmentType{
    equipment_type_id: u8,
    equipment_type_key: Option<String>,
    display_order: u8,
}

impl Default for EquipmentType {
    fn default() -> Self {
        EquipmentType{
            equipment_type_id: 0,
            equipment_type_key: None,
            display_order: 0,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json_str = r#"
        {
          "type": "ExecutableStepDTO",
          "stepId": 9615001364,
          "stepOrder": 1,
          "stepType": {
            "stepTypeId": 1,
            "stepTypeKey": "warmup",
            "displayOrder": 1
          },
          "childStepId": null,
          "description": null,
          "endCondition": {
            "conditionTypeId": 3,
            "conditionTypeKey": "distance",
            "displayOrder": 3,
            "displayable": true
          },
          "endConditionValue": 400.0,
          "preferredEndConditionUnit": {
            "unitId": 1,
            "unitKey": "meter",
            "factor": 100.0
          },
          "endConditionCompare": null,
          "targetType": {
            "workoutTargetTypeId": 1,
            "workoutTargetTypeKey": "no.target",
            "displayOrder": 1
          },
          "targetValueOne": null,
          "targetValueTwo": null,
          "targetValueUnit": null,
          "zoneNumber": null,
          "secondaryTargetType": null,
          "secondaryTargetValueOne": null,
          "secondaryTargetValueTwo": null,
          "secondaryTargetValueUnit": null,
          "secondaryZoneNumber": null,
          "endConditionZone": null,
          "strokeType": {
            "strokeTypeId": 6,
            "strokeTypeKey": "free",
            "displayOrder": 6
          },
          "equipmentType": {
            "equipmentTypeId": 0,
            "equipmentTypeKey": null,
            "displayOrder": 0
          },
          "category": null,
          "exerciseName": null,
          "workoutProvider": null,
          "providerExerciseSourceId": null,
          "weightValue": null,
          "weightUnit": null
        }
        "#;

        let object: ExecutableStepDTO = serde_json::from_str(json_str).unwrap();

        assert_eq!(object.step_id, 9615001364);
        assert_eq!(object.step_order, 1);
        assert_eq!(object.equipment_type.equipment_type_key, None);
    }
}

