use crate::garmin::sport_type::SportType;
use crate::garmin::workout_steps::WorkoutStep;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkoutSegment {
    segment_order: u8,
    sport_type: SportType,
    pool_length_unit: Option<String>,
    pool_length: Option<f32>,
    avg_training_speed: Option<f32>,
    estimated_duration_in_secs: Option<f32>,
    estimated_distance_in_meters: Option<f32>,
    estimated_distance_unit: Option<String>,
    estimated_type: Option<String>,
    description: Option<String>,
    workout_steps: Vec<WorkoutStep>
}