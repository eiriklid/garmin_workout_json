use crate::garmin::workout_steps::end_condition::{Condition, EndCondition};
use crate::garmin::workout_steps::executable_step_dto::ExecutableStepDTO;
use crate::garmin::workout_steps::preferred_end_condition_unit::PreferredEndConditionUnit;
use crate::garmin::workout_steps::step_type::{Step, StepType};
use serde::{Deserialize, Serialize};
use std::cell::Cell;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub struct RepeatGroupDTO {
    step_id: u64,
    step_order: u8,
    step_type: StepType,
    child_step_id: u8,
    number_of_iterations: u8,
    workout_steps: Vec<ExecutableStepDTO>,
    end_condition_value: f32,
    preferred_end_condition_unit: Option<PreferredEndConditionUnit>,
    end_condition_compare: Option<bool>,
    end_condition: EndCondition,
    skip_last_rest_step: Option<bool>,
    smart_repeat: bool,
}

impl RepeatGroupDTO {
    pub fn new(
        step_id: u64,
        step_order: u8,
        child_step_id: u8,
        number_of_iterations: u8,
        mut workout_steps: Vec<ExecutableStepDTO>, // Vec<WorkoutStep>
    ) -> Self {
        // Set child_step_id in workout steps
        for i in workout_steps.iter_mut() {
            i.child_step_id = Cell::new(Some(child_step_id))
        }

        RepeatGroupDTO {
            step_id,
            step_order,
            step_type: StepType {
                step_type_key: Step::Repeat,
            },
            child_step_id,
            number_of_iterations,
            workout_steps,
            end_condition_value: number_of_iterations as f32,

            preferred_end_condition_unit: None,
            end_condition_compare: None,
            end_condition: EndCondition {
                condition_type_key: Condition::Iterations,
                displayable: false,
            },
            skip_last_rest_step: None,
            smart_repeat: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::garmin::workout_steps::step_type::Step;
    use crate::garmin::workout_steps::stroke_type::{Stroke, StrokeType};

    #[test]
    fn test_build_repeat_group_dto() {
        let workout_step = ExecutableStepDTO::active_step(
            9615001364,
            3,
            StepType {
                step_type_key: Step::Warmup,
            },
            None.into(),
            None,
            EndCondition {
                condition_type_key: Condition::Distance,
                displayable: true,
            },
            400.0,
            None,
            StrokeType {
                stroke_type_key: Some(Stroke::Free),
            },
        );

        let step = RepeatGroupDTO::new(9615001366, 3, 1, 8, vec![workout_step]);
        assert_eq!(step.step_id, 9615001366);
        assert_eq!(step.step_order, 3);
        assert_eq!(step.child_step_id, 1);
        assert_eq!(step.number_of_iterations, 8);

        //Check that first entry workout_steps have child_step_id set
        assert_eq!(step.workout_steps[0].child_step_id, Some(1).into());
    }

    #[test]
    fn test_serialize() {
        let workout_steps = vec![
            ExecutableStepDTO::active_step(
                9615001367,
                4,
                StepType {
                    step_type_key: Step::Main,
                },
                None.into(),
                None,
                EndCondition {
                    condition_type_key: Condition::Distance,
                    displayable: true,
                },
                100.0,
                None,
                StrokeType {
                    stroke_type_key: Some(Stroke::Free),
                },
            ),
            ExecutableStepDTO::rest_step(
                9615001368,
                5,
                None.into(),
                None,
                EndCondition {
                    condition_type_key: Condition::FixedRest,
                    displayable: true,
                },
                15.0,
            ),
        ];

        let repeat_group = RepeatGroupDTO::new(9615001366, 3, 1, 8, workout_steps);

        let serialized = serde_json::to_string(&repeat_group).unwrap();
        let expected_json = r#"{
          "type": "RepeatGroupDTO",
          "stepId": 9615001366,
          "stepOrder": 3,
          "stepType": {
            "stepTypeId": 6,
            "stepTypeKey": "repeat",
            "displayOrder": 6
          },
          "childStepId": 1,
          "numberOfIterations": 8,
          "workoutSteps": [
            {
              "type": "ExecutableStepDTO",
              "stepId": 9615001367,
              "stepOrder": 4,
              "stepType": {
                "stepTypeId": 8,
                "stepTypeKey": "main",
                "displayOrder": 8
              },
              "childStepId": 1,
              "description": null,
              "endCondition": {
                "conditionTypeId": 3,
                "conditionTypeKey": "distance",
                "displayOrder": 3,
                "displayable": true
              },
              "endConditionValue": 100.0,
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
            },
            {
              "type": "ExecutableStepDTO",
              "stepId": 9615001368,
              "stepOrder": 5,
              "stepType": {
                "stepTypeId": 5,
                "stepTypeKey": "rest",
                "displayOrder": 5
              },
              "childStepId": 1,
              "description": null,
              "endCondition": {
                "conditionTypeId": 8,
                "conditionTypeKey": "fixed.rest",
                "displayOrder": 8,
                "displayable": true
              },
              "endConditionValue": 15.0,
              "preferredEndConditionUnit": null,
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
                "strokeTypeId": 0,
                "strokeTypeKey": null,
                "displayOrder": 0
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
          ],
          "endConditionValue": 8.0,
          "preferredEndConditionUnit": null,
          "endConditionCompare": null,
          "endCondition": {
            "conditionTypeId": 7,
            "conditionTypeKey": "iterations",
            "displayOrder": 7,
            "displayable": false
          },
          "skipLastRestStep": null,
          "smartRepeat": false
        }
        "#
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
          "type": "RepeatGroupDTO",
          "stepId": 9615001366,
          "stepOrder": 3,
          "stepType": {
            "stepTypeId": 6,
            "stepTypeKey": "repeat",
            "displayOrder": 6
          },
          "childStepId": 1,
          "numberOfIterations": 8,
          "workoutSteps": [
            {
              "type": "ExecutableStepDTO",
              "stepId": 9615001367,
              "stepOrder": 4,
              "stepType": {
                "stepTypeId": 8,
                "stepTypeKey": "main",
                "displayOrder": 8
              },
              "childStepId": 1,
              "description": null,
              "endCondition": {
                "conditionTypeId": 3,
                "conditionTypeKey": "distance",
                "displayOrder": 3,
                "displayable": true
              },
              "endConditionValue": 100.0,
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
            },
            {
              "type": "ExecutableStepDTO",
              "stepId": 9615001368,
              "stepOrder": 5,
              "stepType": {
                "stepTypeId": 5,
                "stepTypeKey": "rest",
                "displayOrder": 5
              },
              "childStepId": 1,
              "description": null,
              "endCondition": {
                "conditionTypeId": 8,
                "conditionTypeKey": "fixed.rest",
                "displayOrder": 8,
                "displayable": true
              },
              "endConditionValue": 15.0,
              "preferredEndConditionUnit": null,
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
                "strokeTypeId": 0,
                "strokeTypeKey": null,
                "displayOrder": 0
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
          ],
          "endConditionValue": 8.0,
          "preferredEndConditionUnit": null,
          "endConditionCompare": null,
          "endCondition": {
            "conditionTypeId": 7,
            "conditionTypeKey": "iterations",
            "displayOrder": 7,
            "displayable": false
          },
          "skipLastRestStep": null,
          "smartRepeat": false
        }
        "#;

        let result: RepeatGroupDTO = serde_json::from_str(&json).unwrap();

        assert_eq!(result.step_id, 9615001366);

        assert_eq!(result.workout_steps.len(), 2);
        assert_eq!(result.workout_steps[1].child_step_id, Cell::new(Some(1)))
    }
}
