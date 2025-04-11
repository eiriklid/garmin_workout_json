use serde::{Deserialize, Serialize};
use crate::garmin::workout_steps::end_condition::EndCondition;
use crate::garmin::workout_steps::executable_step_dto::ExecutableStepDTO;
use crate::garmin::workout_steps::preferred_end_condition_unit::PreferredEndConditionUnit;
use crate::garmin::workout_steps::step_type::StepType;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
struct RepeatGroupDTO {
    /*
    Todo:
        - Test serialize, deserialize
        -
     */
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
    smart_repeat: bool
}

impl RepeatGroupDTO {
    pub fn new(
        step_id: u64,
        step_order: u8,
        child_step_id: u8,
        number_of_iterations: u8,
        mut workout_steps: Vec<ExecutableStepDTO> // Vec<WorkoutStep>
    ) -> Self {

        // Set child_step_id in workout steps
        for i in workout_steps.iter_mut(){ i.child_step_id = Some(child_step_id)};
        

        RepeatGroupDTO {
            step_id,
            step_order,
            step_type: StepType{
                step_type_id: 6,
                step_type_key: "repeat".to_string(),
                display_order: 6
            },
            child_step_id,
            number_of_iterations,
            workout_steps,
            end_condition_value: number_of_iterations as f32,

            preferred_end_condition_unit: None,
            end_condition_compare: None,
            end_condition: EndCondition{
                condition_type_id: 7,
                condition_type_key: "iterations".to_string(),
                display_order: 7,
                displayable: false,
            },
            skip_last_rest_step: None,
            smart_repeat: false,
        }
        }
}

#[cfg(test)]
mod tests {
    use crate::garmin::workout_steps::stroke_type::StrokeType;
    use super::*;

    #[test]
    fn test_build_repeat_group_dto() {
        let workout_step = ExecutableStepDTO::new(
            9615001364,
            3,
            StepType{
                step_type_id: 1,
                step_type_key: "warmup".to_string(),
                display_order: 1
            },
            None,
            None,
            EndCondition{
                condition_type_id: 3,
                condition_type_key: "distance".to_string(),
                display_order: 3,
                displayable: true
            },
            400.0,
            None,
            StrokeType{
                stroke_type_id: 6,
                stroke_type_key: Some("free".to_string()),
                display_order: 6
            }
        );


        let step = RepeatGroupDTO::new(
            9615001366,
            3,
            1,
            8,
            vec![workout_step]
        );
        assert_eq!(step.step_id, 9615001366);
        assert_eq!(step.step_order, 3);
        assert_eq!(step.child_step_id, 1);
        assert_eq!(step.number_of_iterations, 8);

        //Check that first entry workout_steps have child_step_id set
        assert_eq!(step.workout_steps[0].child_step_id, Some(1));
    }
}
