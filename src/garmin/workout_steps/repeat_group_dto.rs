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
        - Test
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
        workout_steps: Vec<ExecutableStepDTO>
    ) -> Self {
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