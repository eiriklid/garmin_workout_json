use crate::garmin::unit::Unit;
use crate::garmin::workout::Workout;
use crate::garmin::workout_segments::WorkoutSegment;
use crate::garmin::workout_steps::end_condition::{Condition, EndCondition};
use crate::garmin::workout_steps::executable_step_dto::ExecutableStepDTO;
use crate::garmin::workout_steps::repeat_group_dto::RepeatGroupDTO;
use crate::garmin::workout_steps::step_type::{Step, StepType};
use crate::garmin::workout_steps::stroke_type::{Stroke, StrokeType};
use crate::garmin::workout_steps::WorkoutStep;
use chrono::{NaiveDateTime, Utc};
use std::cell::Cell;

const STEP_OFFSET: u64 = 9615001364;

pub(crate) fn main(){
    // Set timestamps
    // Todo: Figure out if created date can be updated

    let utc: NaiveDateTime = Utc::now().naive_utc();

    let workout = Workout::new_swimming_workout(
        1180301830, //Update
        100441918,
        "Core Workout 4".to_string(),
        None,
        utc.to_string(),
        utc.to_string(),
        vec![
            WorkoutSegment::new(
                vec![
                    WorkoutStep::Repeat(RepeatGroupDTO::new(
                        1 + STEP_OFFSET,
                        1,
                        1,
                        2,
                        vec![
                            ExecutableStepDTO::new(
                                2 + STEP_OFFSET,
                                2,
                                StepType::new(Step::Warmup),
                                Cell::new(Some(1)),
                                None,
                                EndCondition::new(Condition::Distance),
                                50.0,
                                None,
                                StrokeType::new(Some(Stroke::AnyStroke))
                            ),
                            ExecutableStepDTO::rest_step(
                                3 + STEP_OFFSET,
                                3,
                                Cell::new(Some(1)),
                                None,
                                EndCondition::new(Condition::Time),
                                20.0
                            )
                        ]
                    ))
                ]
            )
        ],
        25.0,
        Unit::default()

    );

    // Write workout to json file
    serde_json::to_writer_pretty(std::io::stdout(), &workout).unwrap();

}