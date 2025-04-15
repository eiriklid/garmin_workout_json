use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize)]
struct Workout {
    workoutId: u64,
    ownerId: u64,
    workoutName: String,
    description: Option<String>,
    updatedDate: Option<String>,
    createdDate: Option<String>,
    sportType: SportType,
    subSportType: Option<String>,
    trainingPlanId: Option<String>,
    author: Author,
    sharedWithUsers: Option<String>,
    estimatedDurationInSecs: Option<u32>,
    estimatedDistanceInMeters: Option<f64>,
    workoutSegments: Vec<WorkoutSegment>,
    poolLength: Option<f64>,
    poolLengthUnit: Option<Unit>,
    locale: Option<String>,
    workoutProvider: Option<String>,
    workoutSourceId: Option<String>,
    uploadTimestamp: Option<String>,
    atpPlanId: Option<String>,
    consumer: Option<String>,
    consumerName: Option<String>,
    consumerImageURL: Option<String>,
    consumerWebsiteURL: Option<String>,
    workoutNameI18nKey: Option<String>,
    descriptionI18nKey: Option<String>,
    avgTrainingSpeed: Option<f64>,
    estimateType: Option<String>,
    estimatedDistanceUnit: Option<Unit>,
    workoutThumbnailUrl: Option<String>,
    isSessionTransitionEnabled: Option<bool>,
    shared: bool,
}

#[derive(Serialize)] struct SportType { sportTypeId: u32, sportTypeKey: String, displayOrder: u32, }
#[derive(Serialize)] struct Author { userProfilePk: u64, displayName: String, fullName: String, profileImgNameLarge: Option<String>, profileImgNameMedium: Option<String>, profileImgNameSmall: Option<String>, userPro: bool, vivokidUser: bool, }
#[derive(Serialize)] struct WorkoutSegment { segmentOrder: u32, sportType: SportType, poolLengthUnit: Option<String>, poolLength: Option<f64>, avgTrainingSpeed: Option<f64>, estimatedDurationInSecs: Option<u32>, estimatedDistanceInMeters: Option<f64>, estimatedDistanceUnit: Option<String>, estimateType: Option<String>, description: Option<String>, workoutSteps: Vec<WorkoutStep>, }
#[derive(Serialize)]
#[serde(tag = "type")]
enum WorkoutStep {
    ExecutableStepDTO {
        stepId: u64,
        stepOrder: u32,
        stepType: StepType,
        childStepId: Option<u32>,
        description: Option<String>,
        endCondition: EndCondition,
        endConditionValue: f64,
        preferredEndConditionUnit: Option<Unit>,
        endConditionCompare: Option<String>,
        targetType: TargetType,
        targetValueOne: Option<f64>,
        targetValueTwo: Option<f64>,
        targetValueUnit: Option<Unit>,
        zoneNumber: Option<u32>,
        secondaryTargetType: Option<TargetType>,
        secondaryTargetValueOne: Option<f64>,
        secondaryTargetValueTwo: Option<f64>,
        secondaryTargetValueUnit: Option<Unit>,
        secondaryZoneNumber: Option<u32>,
        endConditionZone: Option<u32>,
        strokeType: StrokeType,
        equipmentType: EquipmentType,
        category: Option<String>,
        exerciseName: Option<String>,
        workoutProvider: Option<String>,
        providerExerciseSourceId: Option<String>,
        weightValue: Option<f64>,
        weightUnit: Option<Unit>,
    },
    RepeatGroupDTO {
        stepId: u64,
        stepOrder: u32,
        stepType: StepType,
        childStepId: u32,
        numberOfIterations: u32,
        workoutSteps: Vec<WorkoutStep>,
        endCondition: EndCondition,
        endConditionCompare: Option<String>,
        endConditionValue: f64,
        preferredEndConditionUnit: Option<Unit>,
        skipLastRestStep: Option<bool>,
        smartRepeat: bool,
    },
}
#[derive(Serialize)] struct StepType { stepTypeId: u32, stepTypeKey: String, displayOrder: u32, }
#[derive(Serialize)] struct EndCondition { conditionTypeId: u32, conditionTypeKey: String, displayOrder: u32, displayable: bool, }
#[derive(Serialize)] struct Unit { unitId: Option<u32>, unitKey: Option<String>, factor: Option<f64>, }
#[derive(Serialize)] struct StrokeType { strokeTypeId: u32, strokeTypeKey: Option<String>, displayOrder: u32, }
#[derive(Serialize)] struct EquipmentType { equipmentTypeId: u32, equipmentTypeKey: Option<String>, displayOrder: u32, }
#[derive(Serialize)]
struct TargetType {
    workoutTargetTypeId: u32,
    workoutTargetTypeKey: String,
    displayOrder: u32,
}

pub trait InputProvider {
    fn get_input(&self, prompt: &str) -> String;
}

pub struct ConsoleInput;

impl InputProvider for ConsoleInput {
    fn get_input(&self, prompt: &str) -> String {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

pub struct WorkoutBuilder {
    input_provider: Box<dyn InputProvider>,
}

impl WorkoutBuilder {
    pub fn new(input_provider: Box<dyn InputProvider>) -> Self {
        Self { input_provider }
    }

    fn get_pool_length(&self) -> f64 {
        loop {
            println!("Select pool length:");
            println!("1) 25m pool");
            println!("2) 17m pool");
            
            match self.input_provider.get_input("Enter choice (1 or 2):").as_str() {
                "1" => return 25.0,
                "2" => return 17.0,
                _ => println!("Invalid choice, please try again"),
            }
        }
    }

    fn get_step_type(&self) -> String {
        loop {
            println!("Select step type:");
            println!("1) Warmup");
            println!("2) Main");
            println!("3) Cooldown");
            println!("4) Rest");
            
            match self.input_provider.get_input("Enter choice (1-4):").as_str() {
                "1" => return "warmup".to_string(),
                "2" => return "main".to_string(),
                "3" => return "cooldown".to_string(),
                "4" => return "rest".to_string(),
                _ => println!("Invalid choice, please try again"),
            }
        }
    }

    fn get_stroke_type(&self) -> String {
        loop {
            println!("Select stroke type:");
            println!("1) Freestyle");
            println!("2) Backstroke");
            println!("3) Breaststroke");
            println!("4) Butterfly");
            println!("5) Individual Medley");
            
            match self.input_provider.get_input("Enter choice (1-5):").as_str() {
                "1" => return "free".to_string(),
                "2" => return "back".to_string(),
                "3" => return "breast".to_string(),
                "4" => return "fly".to_string(),
                "5" => return "im".to_string(),
                _ => println!("Invalid choice, please try again"),
            }
        }
    }

    fn get_segment_definition(&self) -> Option<SegmentDefinition> {
        println!("\n=== New Segment ===");
        let continue_input = self.input_provider.get_input("Add a new segment? (y/n): ");
        if continue_input.to_lowercase() != "y" {
            return None;
        }

        let repetitions = self.input_provider
            .get_input("Enter number of repetitions: ")
            .parse()
            .unwrap_or(1);

        let step_type = self.get_step_type();
        let stroke = self.get_stroke_type();
        
        let rest_time = self.input_provider
            .get_input("Enter rest time in seconds: ")
            .parse()
            .unwrap_or(30.0);

        let distance = self.input_provider
            .get_input("Enter distance for this segment in meters: ")
            .parse()
            .unwrap_or(100.0);

        Some(SegmentDefinition {
            repetitions,
            step_type,
            stroke,
            rest_time,
            distance,
        })
    }

    fn create_segment_from_definition(&self, order: u32, def: &SegmentDefinition) -> WorkoutSegment {
        let mut steps = Vec::new();
        
        // Create warmup step
        steps.push(WorkoutStep::ExecutableStepDTO {
            stepId: 9615001364,
            stepOrder: 1,
            stepType: StepType { 
                stepTypeId: 1, 
                stepTypeKey: "warmup".to_string(), 
                displayOrder: 1 
            },
            childStepId: None,
            description: None,
            endCondition: EndCondition { 
                conditionTypeId: 3, 
                conditionTypeKey: Condition::Distance,
                displayOrder: 3, 
                displayable: true 
            },
            endConditionValue: 400.0,
            preferredEndConditionUnit: Some(Unit {
                unitId: Some(1),
                unitKey: Some("meter".to_string()),
                factor: Some(100.0),
            }),
            endConditionCompare: None,
            targetType: TargetType {
                workoutTargetTypeId: 1,
                workoutTargetTypeKey: "no.target".to_string(),
                displayOrder: 1,
            },
            targetValueOne: None,
            targetValueTwo: None,
            targetValueUnit: None,
            zoneNumber: None,
            secondaryTargetType: None,
            secondaryTargetValueOne: None,
            secondaryTargetValueTwo: None,
            secondaryTargetValueUnit: None,
            secondaryZoneNumber: None,
            endConditionZone: None,
            strokeType: StrokeType { 
                strokeTypeId: 6, 
                strokeTypeKey: Some("free".to_string()), 
                displayOrder: 6 
            },
            equipmentType: EquipmentType { 
                equipmentTypeId: 0, 
                equipmentTypeKey: None, 
                displayOrder: 0 
            },
            category: None,
            exerciseName: None,
            workoutProvider: None,
            providerExerciseSourceId: None,
            weightValue: None,
            weightUnit: None,
        });

        // Create rest step
        steps.push(WorkoutStep::ExecutableStepDTO {
            stepId: 9615001365,
            stepOrder: 2,
            stepType: StepType { 
                stepTypeId: 5, 
                stepTypeKey: "rest".to_string(), 
                displayOrder: 5 
            },
            childStepId: None,
            description: None,
            endCondition: EndCondition { 
                conditionTypeId: 1, 
                conditionTypeKey: "lap.button".to_string(), 
                displayOrder: 1, 
                displayable: true 
            },
            endConditionValue: 200.0,
            preferredEndConditionUnit: None,
            endConditionCompare: None,
            targetType: TargetType {
                workoutTargetTypeId: 1,
                workoutTargetTypeKey: "no.target".to_string(),
                displayOrder: 1,
            },
            targetValueOne: None,
            targetValueTwo: None,
            targetValueUnit: None,
            zoneNumber: None,
            secondaryTargetType: None,
            secondaryTargetValueOne: None,
            secondaryTargetValueTwo: None,
            secondaryTargetValueUnit: None,
            secondaryZoneNumber: None,
            endConditionZone: None,
            strokeType: StrokeType { 
                strokeTypeId: 0, 
                strokeTypeKey: None, 
                displayOrder: 0 
            },
            equipmentType: EquipmentType { 
                equipmentTypeId: 0, 
                equipmentTypeKey: None, 
                displayOrder: 0 
            },
            category: None,
            exerciseName: None,
            workoutProvider: None,
            providerExerciseSourceId: None,
            weightValue: None,
            weightUnit: None,
        });

        // Create repeat group
        let mut repeat_steps = Vec::new();
        
        // Main set step
        repeat_steps.push(WorkoutStep::ExecutableStepDTO {
            stepId: 9615001367,
            stepOrder: 4,
            stepType: StepType { 
                stepTypeId: 8, 
                stepTypeKey: "main".to_string(), 
                displayOrder: 8 
            },
            childStepId: Some(1),
            description: None,
            endCondition: EndCondition { 
                conditionTypeId: 3, 
                conditionTypeKey: Condition::Distance,
                displayOrder: 3, 
                displayable: true 
            },
            endConditionValue: 100.0,
            preferredEndConditionUnit: Some(Unit {
                unitId: Some(1),
                unitKey: Some("meter".to_string()),
                factor: Some(100.0),
            }),
            endConditionCompare: None,
            targetType: TargetType {
                workoutTargetTypeId: 1,
                workoutTargetTypeKey: "no.target".to_string(),
                displayOrder: 1,
            },
            targetValueOne: None,
            targetValueTwo: None,
            targetValueUnit: None,
            zoneNumber: None,
            secondaryTargetType: None,
            secondaryTargetValueOne: None,
            secondaryTargetValueTwo: None,
            secondaryTargetValueUnit: None,
            secondaryZoneNumber: None,
            endConditionZone: None,
            strokeType: StrokeType { 
                strokeTypeId: 6, 
                strokeTypeKey: Some("free".to_string()), 
                displayOrder: 6 
            },
            equipmentType: EquipmentType { 
                equipmentTypeId: 0, 
                equipmentTypeKey: None, 
                displayOrder: 0 
            },
            category: None,
            exerciseName: None,
            workoutProvider: None,
            providerExerciseSourceId: None,
            weightValue: None,
            weightUnit: None,
        });

        // Rest step in repeat group
        repeat_steps.push(WorkoutStep::ExecutableStepDTO {
            stepId: 9615001368,
            stepOrder: 5,
            stepType: StepType { 
                stepTypeId: 5, 
                stepTypeKey: "rest".to_string(), 
                displayOrder: 5 
            },
            childStepId: Some(1),
            description: None,
            endCondition: EndCondition { 
                conditionTypeId: 8, 
                conditionTypeKey: "fixed.rest".to_string(), 
                displayOrder: 8, 
                displayable: true 
            },
            endConditionValue: def.rest_time,
            preferredEndConditionUnit: None,
            endConditionCompare: None,
            targetType: TargetType {
                workoutTargetTypeId: 1,
                workoutTargetTypeKey: "no.target".to_string(),
                displayOrder: 1,
            },
            targetValueOne: None,
            targetValueTwo: None,
            targetValueUnit: None,
            zoneNumber: None,
            secondaryTargetType: None,
            secondaryTargetValueOne: None,
            secondaryTargetValueTwo: None,
            secondaryTargetValueUnit: None,
            secondaryZoneNumber: None,
            endConditionZone: None,
            strokeType: StrokeType { 
                strokeTypeId: 0, 
                strokeTypeKey: None, 
                displayOrder: 0 
            },
            equipmentType: EquipmentType { 
                equipmentTypeId: 0, 
                equipmentTypeKey: None, 
                displayOrder: 0 
            },
            category: None,
            exerciseName: None,
            workoutProvider: None,
            providerExerciseSourceId: None,
            weightValue: None,
            weightUnit: None,
        });

        // Add repeat group
        steps.push(WorkoutStep::RepeatGroupDTO {
            stepId: 9615001366,
            stepOrder: 3,
            stepType: StepType { 
                stepTypeId: 6, 
                stepTypeKey: "repeat".to_string(), 
                displayOrder: 6 
            },
            childStepId: 1,
            numberOfIterations: 8,
            workoutSteps: repeat_steps,
            endCondition: EndCondition {
                conditionTypeId: 7,
                conditionTypeKey: "iterations".to_string(),
                displayOrder: 7,
                displayable: false,
            },
            endConditionCompare: None,
            endConditionValue: 8.0,
            preferredEndConditionUnit: None,
            skipLastRestStep: None,
            smartRepeat: false,
        });

        // Add rest step after repeat group
        steps.push(WorkoutStep::ExecutableStepDTO {
            stepId: 9615001369,
            stepOrder: 6,
            stepType: StepType { 
                stepTypeId: 5, 
                stepTypeKey: "rest".to_string(), 
                displayOrder: 5 
            },
            childStepId: None,
            description: None,
            endCondition: EndCondition { 
                conditionTypeId: 1, 
                conditionTypeKey: "lap.button".to_string(), 
                displayOrder: 1, 
                displayable: true 
            },
            endConditionValue: 200.0,
            preferredEndConditionUnit: None,
            endConditionCompare: None,
            targetType: TargetType {
                workoutTargetTypeId: 1,
                workoutTargetTypeKey: "no.target".to_string(),
                displayOrder: 1,
            },
            targetValueOne: None,
            targetValueTwo: None,
            targetValueUnit: None,
            zoneNumber: None,
            secondaryTargetType: None,
            secondaryTargetValueOne: None,
            secondaryTargetValueTwo: None,
            secondaryTargetValueUnit: None,
            secondaryZoneNumber: None,
            endConditionZone: None,
            strokeType: StrokeType { 
                strokeTypeId: 0, 
                strokeTypeKey: None, 
                displayOrder: 0 
            },
            equipmentType: EquipmentType { 
                equipmentTypeId: 0, 
                equipmentTypeKey: None, 
                displayOrder: 0 
            },
            category: None,
            exerciseName: None,
            workoutProvider: None,
            providerExerciseSourceId: None,
            weightValue: None,
            weightUnit: None,
        });

        // Add cooldown step
        steps.push(WorkoutStep::ExecutableStepDTO {
            stepId: 9615001370,
            stepOrder: 7,
            stepType: StepType { 
                stepTypeId: 2, 
                stepTypeKey: "cooldown".to_string(), 
                displayOrder: 2 
            },
            childStepId: None,
            description: None,
            endCondition: EndCondition { 
                conditionTypeId: 3, 
                conditionTypeKey: Condition::Distance,
                displayOrder: 3, 
                displayable: true 
            },
            endConditionValue: 200.0,
            preferredEndConditionUnit: Some(Unit {
                unitId: Some(1),
                unitKey: Some("meter".to_string()),
                factor: Some(100.0),
            }),
            endConditionCompare: None,
            targetType: TargetType {
                workoutTargetTypeId: 1,
                workoutTargetTypeKey: "no.target".to_string(),
                displayOrder: 1,
            },
            targetValueOne: None,
            targetValueTwo: None,
            targetValueUnit: None,
            zoneNumber: None,
            secondaryTargetType: Some(TargetType {
                workoutTargetTypeId: 18,
                workoutTargetTypeKey: "swim.instruction".to_string(),
                displayOrder: 18,
            }),
            secondaryTargetValueOne: Some(3.0),
            secondaryTargetValueTwo: Some(0.0),
            secondaryTargetValueUnit: None,
            secondaryZoneNumber: None,
            endConditionZone: None,
            strokeType: StrokeType { 
                strokeTypeId: 1, 
                strokeTypeKey: Some("any_stroke".to_string()), 
                displayOrder: 1 
            },
            equipmentType: EquipmentType { 
                equipmentTypeId: 0, 
                equipmentTypeKey: None, 
                displayOrder: 0 
            },
            category: None,
            exerciseName: None,
            workoutProvider: None,
            providerExerciseSourceId: None,
            weightValue: None,
            weightUnit: None,
        });

        WorkoutSegment {
            segmentOrder: order,
            sportType: SportType { 
                sportTypeId: 4, 
                sportTypeKey: "swimming".to_string(), 
                displayOrder: 3 
            },
            poolLengthUnit: None,
            poolLength: None,
            avgTrainingSpeed: None,
            estimatedDurationInSecs: None,
            estimatedDistanceInMeters: None,
            estimatedDistanceUnit: None,
            estimateType: None,
            description: None,
            workoutSteps: steps,
        }
    }

    pub fn create_workout(&self) -> String {
        let workout_id: u64 = self.input_provider.get_input("Enter workout ID:").parse().unwrap_or(0);
        let workout_name = self.input_provider.get_input("Enter workout name:");
        let description = self.input_provider.get_input("Enter workout description:");
        let description = if description.is_empty() { None } else { Some(description) };

        let mut def = WorkoutDefinition {
            workout_id,
            workout_name,
            description,
            segments: Vec::new(),
        };

        let _pool_length = self.get_pool_length();

        loop {
            let continue_input = self.input_provider.get_input("Add a new segment? (y/n): ");
            if continue_input.trim().to_lowercase() != "y" {
                break;
            }

            if let Some(segment) = self.get_segment_definition() {
                def.segments.push(segment);
            }
        }

        let workout = self.create_workout_with_data(&def);
        serde_json::to_string_pretty(&workout).unwrap()
    }

    fn create_workout_with_data(&self, def: &WorkoutDefinition) -> Workout {
        let mut workout = Workout {
            workoutId: def.workout_id,
            ownerId: 100441918,
            workoutName: def.workout_name.clone(),
            description: def.description.clone(),
            workoutNameI18nKey: None,
            descriptionI18nKey: None,
            updatedDate: None,
            createdDate: None,
            sportType: SportType {
                sportTypeId: 4,
                sportTypeKey: "swimming".to_string(),
                displayOrder: 3,
            },
            subSportType: None,
            poolLength: Some(25.0),
            poolLengthUnit: Some(Unit {
                unitId: Some(1),
                unitKey: Some("meter".to_string()),
                factor: Some(100.0),
            }),
            workoutSegments: vec![],
            author: Author {
                userProfilePk: 100441918,
                displayName: "ea022d71-d096-4261-a5cd-9fb3aa2bf953".to_string(),
                fullName: "Eirik Vårli Lid".to_string(),
                profileImgNameLarge: None,
                profileImgNameMedium: Some("ff381b64-b337-4dd4-8b7e-3ca04c1b0528-100441918.png".to_string()),
                profileImgNameSmall: Some("bce65d55-03d1-427f-a603-ed8e9a545985-100441918.png".to_string()),
                userPro: false,
                vivokidUser: false,
            },
            estimatedDistanceInMeters: Some(1400.0),
            estimatedDistanceUnit: Some(Unit {
                unitId: None,
                unitKey: None,
                factor: None,
            }),
            estimatedDurationInSecs: Some(0),
            estimateType: Some("TIME_ESTIMATED".to_string()),
            avgTrainingSpeed: Some(0.8333),
            workoutProvider: None,
            consumer: None,
            consumerName: None,
            consumerImageURL: None,
            consumerWebsiteURL: None,
            atpPlanId: None,
            workoutSourceId: None,
            trainingPlanId: None,
            workoutThumbnailUrl: None,
            shared: false,
            sharedWithUsers: None,
            isSessionTransitionEnabled: None,
            locale: None,
            uploadTimestamp: None,
        };

        let mut order = 1;
        for segment_def in &def.segments {
            let segment = self.create_segment_from_definition(order, segment_def);
            workout.workoutSegments.push(segment);
            order += 1;
        }

        workout
    }
}

#[derive(Debug)]
pub struct SegmentDefinition {
    repetitions: u32,
    step_type: String,
    stroke: String,
    rest_time: f64,
    distance: f64,
}

#[derive(Debug)]
struct WorkoutDefinition {
    workout_id: u64,
    workout_name: String,
    description: Option<String>,
    segments: Vec<SegmentDefinition>,
}

fn main() {
    let input_provider = Box::new(ConsoleInput);
    let workout_builder = WorkoutBuilder::new(input_provider);
    let json_output = workout_builder.create_workout();

    let mut input = String::new();
    println!("Enter filename to save the JSON (e.g. workout.json):");
    io::stdin().read_line(&mut input).unwrap();
    let filename = input.trim();

    let mut file = File::create(&filename).expect("Could not create file");
    file.write_all(json_output.as_bytes()).expect("Could not write to file");

    println!("Workout JSON saved to {}", filename);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockInput {
        responses: Vec<String>,
        index: std::cell::Cell<usize>,
    }

    impl MockInput {
        fn new(responses: Vec<String>) -> Self {
            Self {
                responses,
                index: std::cell::Cell::new(0),
            }
        }
    }

    impl InputProvider for MockInput {
        fn get_input(&self, _prompt: &str) -> String {
            let index = self.index.get();
            self.index.set(index + 1);
            self.responses[index].clone()
        }
    }

    #[test]
    fn test_workout_creation() {
        // Simulate user inputs for:
        // - workout ID
        // - workout name
        // - description
        // - pool length choice
        // - first segment (y)
        // - repetitions
        // - step type
        // - stroke type
        // - rest time
        // - distance
        // - second segment (n)
        let mock_responses = vec![
            "1180301830".to_string(),    // workout ID
            "Test Workout".to_string(),   // workout name
            "Test Description".to_string(), // description
            "1".to_string(),             // pool length (25m)
            "y".to_string(),             // add first segment
            "3".to_string(),             // 3 repetitions
            "2".to_string(),             // main set (option 2)
            "1".to_string(),             // freestyle (option 1)
            "30".to_string(),            // 30 seconds rest
            "100".to_string(),           // 100m distance
            "n".to_string(),             // no more segments
        ];
        
        let mock_input = Box::new(MockInput::new(mock_responses));
        let workout_builder = WorkoutBuilder::new(mock_input);
        
        let result = workout_builder.create_workout();
        let parsed: Value = serde_json::from_str(&result).expect("Failed to parse JSON");

        // Test basic workout properties
        assert_eq!(parsed["workoutId"], 1180301830);
        assert_eq!(parsed["workoutName"], "Test Workout");
        assert_eq!(parsed["description"], "Test Description");
        assert_eq!(parsed["poolLength"], 25.0);
        assert_eq!(parsed["estimatedDistanceInMeters"], 100.0);

        // Test segment structure
        let segments = parsed["workoutSegments"].as_array().unwrap();
        assert_eq!(segments.len(), 1);

        // Test first segment
        let first_segment = &segments[0];
        assert_eq!(first_segment["segmentOrder"], 1);
        
        // Test steps in first segment
        let steps = first_segment["workoutSteps"].as_array().unwrap();
        assert_eq!(steps.len(), 5); // 3 activity steps + 2 rest steps

        // Test first activity step
        let first_step = &steps[0];
        assert_eq!(first_step["stepType"]["stepTypeKey"], "main");
        assert_eq!(first_step["strokeType"]["strokeTypeKey"], "free");
        assert_eq!(first_step["endConditionValue"], 100.0 / 3.0); // distance per repetition
    }

    #[test]
    fn test_multiple_segments() {
        let mock_responses = vec![
            "1180301830".to_string(),    // workout ID
            "Multi-Segment Workout".to_string(), // workout name
            "Test Description".to_string(), // description
            "1".to_string(),             // pool length (25m)
            // First segment
            "y".to_string(),             // add first segment
            "4".to_string(),             // 4 repetitions
            "2".to_string(),             // main set (option 2)
            "3".to_string(),             // breaststroke (option 3)
            "30".to_string(),            // 30 seconds rest
            "100".to_string(),           // 100m distance
            "n".to_string(),             // no more segments
        ];
        
        let mock_input = Box::new(MockInput::new(mock_responses));
        let workout_builder = WorkoutBuilder::new(mock_input);
        
        let result = workout_builder.create_workout();
        let parsed: Value = serde_json::from_str(&result).expect("Failed to parse JSON");

        // Test basic workout properties
        assert_eq!(parsed["workoutName"], "Multi-Segment Workout");
        assert_eq!(parsed["estimatedDistanceInMeters"], 300.0); // 200m + 100m

        // Test segments
        let segments = parsed["workoutSegments"].as_array().unwrap();
        assert_eq!(segments.len(), 2);

        // Test first segment (warmup)
        let first_segment = &segments[0];
        assert_eq!(first_segment["segmentOrder"], 1);
        let first_segment_steps = first_segment["workoutSteps"].as_array().unwrap();
        assert_eq!(first_segment_steps.len(), 5); // 2 activity steps + 3 rest steps
    }

    #[test]
    fn test_exact_workout_recreation() {
        use std::fs;

        // Read the expected workout file
        let expected = fs::read_to_string("src/expected_workout.json")
            .expect("Missing expected file");
        let expected_json: Value = serde_json::from_str(&expected)
            .expect("Invalid JSON in expected file");

        // Extract the necessary information from the expected workout
        let mock_responses = vec![
            expected_json["workoutId"].to_string(),           // workout ID
            expected_json["workoutName"].to_string().replace("\"", ""), // workout name
            "".to_string(),                                   // description (none in expected)
            "1".to_string(),                                  // 25m pool
            // First segment
            "y".to_string(),                                  // add segment
            "3".to_string(),                                  // 3 repetitions
            "2".to_string(),                                  // main set
            "1".to_string(),                                  // freestyle
            "15".to_string(),                                 // 15 seconds rest
            "1400".to_string(),                              // 1400m total distance
            "n".to_string(),                                  // no more segments
        ];

        let mock_input = Box::new(MockInput::new(mock_responses));
        let workout_builder = WorkoutBuilder::new(mock_input);
        
        let result = workout_builder.create_workout();
        let actual_json: Value = serde_json::from_str(&result)
            .expect("Invalid JSON from generator");

        // Function to remove timestamp fields for comparison
        fn remove_timestamps(mut value: Value) -> Value {
            if let Value::Object(ref mut map) = value {
                map.remove("updatedDate");
                map.remove("createdDate");
            }
            value
        }

        fn pretty_json_diff(expected: &Value, actual: &Value) -> String {
            use std::fmt::Write;
            let mut diff = String::new();
            
            if expected != actual {
                writeln!(&mut diff, "Expected:").unwrap();
                writeln!(&mut diff, "{}", serde_json::to_string_pretty(expected).unwrap()).unwrap();
                writeln!(&mut diff, "\nActual:").unwrap();
                writeln!(&mut diff, "{}", serde_json::to_string_pretty(actual).unwrap()).unwrap();
            }
            
            diff
        }

        // Compare JSONs without timestamps
        let expected_without_timestamps = remove_timestamps(expected_json);
        let actual_without_timestamps = remove_timestamps(actual_json);

        assert_eq!(
            expected_without_timestamps, 
            actual_without_timestamps,
            "Generated workout does not match expected workout:\n{}",
            pretty_json_diff(&expected_without_timestamps, &actual_without_timestamps)
        );
    }
}
