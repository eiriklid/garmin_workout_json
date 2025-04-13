use crate::garmin::author::Author;
use crate::garmin::sport_type::SportType;
use crate::garmin::unit::Unit;
use crate::garmin::workout_segments::WorkoutSegment;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workout {
    /*
    Todo:
        - Test complete generation
        - DateTime representation
     */
    workout_id: u64,
    owner_id: u64,
    workout_name: String,
    description: Option<String>,
    updated_date: String, // Find datetime representation
    created_date: String, // Find datetime representation
    sport_type: SportType,
    sub_sport_type: Option<SportType>,
    training_plan_id: Option<u64>,
    author: Author,
    shared_with_users: Option<bool>,
    estimated_duration_in_secs: u32,
    estimated_distance_in_meters: Option<f32>,
    workout_segments: Vec<WorkoutSegment>,
    pool_length: f32,
    pool_length_unit: Unit,
    locale: Option<String>,
    workout_provider: Option<String>,
    workout_source_id: Option<String>,
    upload_timestamp: Option<String>,
    atp_plan_id: Option<String>,
    consumer: Option<String>,
    consumer_name: Option<String>,
    consumer_image_u_r_l: Option<String>,
    consumer_website_u_r_l: Option<String>,
    workout_name_i18n_key: Option<String>,
    description_i18n_key: Option<String>,
    avg_training_speed: Option<f32>,
    estimate_type: String,
    estimated_distance_unit: Unit,
    workout_thumbnail_url: Option<String>,
    is_session_transition_enabled: Option<String>,
    shared: bool
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_deserialize() {
        let json_string = fs::read_to_string("src/expected_workout.json")
            .expect("Missing expected file");
        let workout: Workout = serde_json::from_str(&json_string)
            .expect("Invalid JSON in expected file");

        assert_eq!(workout.workout_id, 1180301830);
        assert_eq!(workout.owner_id, 100441918);
        assert_eq!(workout.pool_length, 25.0);
        assert_eq!(workout.estimated_distance_in_meters, Some(1400.0));
    }
}