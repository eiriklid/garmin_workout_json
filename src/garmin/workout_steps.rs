use serde::{Deserialize, Serialize};

pub mod step_type;
pub mod end_condition;
pub mod target_type;
mod preferred_end_condition_unit;
pub mod stroke_type;
pub mod executable_step_dto;
pub mod repeat_group_dto;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum WorkoutStep {
    ExecutableStepDTO,
    RepeatGroupDTO,
}