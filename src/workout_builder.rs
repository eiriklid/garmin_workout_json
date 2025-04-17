use crate::garmin::workout::Workout;
use crate::garmin::workout_steps::end_condition::{Condition, EndCondition};
use crate::garmin::workout_steps::executable_step_dto::ExecutableStepDTO;
use crate::garmin::workout_steps::step_type::{Step, StepType};
use crate::garmin::workout_steps::stroke_type::{Stroke, StrokeType};
use crate::garmin::workout_steps::WorkoutStep;

use crate::garmin::workout_steps::target_type::TargetType;
use std::io;
use std::io::Write;
use std::str::FromStr;


const STEP_OFFSET: usize = 9615001364;

fn get_input<T: FromStr>(prompt: &str) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed = input.trim();
        match trimmed.parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input, please try again."),
        }
    }
}



pub struct WorkoutBuilder {}


impl WorkoutBuilder {

    pub fn new_workout(&self) -> Workout{
        let workout_name = self.get_workout_name();

        let pool_length = self.get_pool_length();

        let mut workout_steps: Vec<WorkoutStep> = Vec::new();


        let step_counter = 1;
        loop {
            let workout_step_type = self.get_single_or_repeat();

            let step_type = self.get_step_type();
            let stroke_type = self.get_stroke_type();
            let end_condition = self.get_end_condition();
            let end_value: f32 = get_input("End value: "); // + &end_condition;

            // I think if end_condition is Distance, PreferredEndConditionUnit must be set
            // Todo: Verify the combinations, and create a getter corresponding to End condition
            // let end_unit = ...

            let target_type = match end_condition.condition_type_key {
                Condition::LapButton => Some(TargetType::default()),
                _ => None
            };

            let executable_step = match workout_step_type {
                RepeatGroupDTO => {
                    let number_of_iterations: u8 = get_input("Number of iterations: ");
                },
                ExecutableStepDTO => {
                    ExecutableStepDTO::new(
                        step_id: step_counter + STEP_OFFSET,
                        step_counter,
                        step_type,
                    );
                }
            };

            let choice = loop {
                let mut input = get_input("Add another step? (Y/N)");
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => { break true },
                    "n" | "no" => { break false },
                    _ => {
                        println!("invalid choice, try again!");
                        continue;
                    },
                }
            };
            if choice{
                // Add rest_step
                break
            }
        }

        Workout::new_swimming_workout(
            1180301830, // Update
            100441918,
            workout_name,
            None,

        )
    }

    fn get_workout_name(&self) -> String {
        // Get name of workout
        let mut workout_name = String::new();
        io::stdin().read_line(&mut workout_name).unwrap();
        workout_name.trim().to_string()
    }

    fn get_pool_length(&self) -> f64 {
        println!("Select pool length:");
        println!("1) 25m pool");
        println!("2) 17m pool");
        println!("3) Custom length");
        loop {
            let mut input = get_input("Enter choice (1, 2 or 3):");
            match input.trim() {
                "1" => return 25.0,
                "2" => return 17.0,
                "3" => return self.get_custom_pool_length(),
                _ => println!("Invalid choice, please try again"),
            }
        }
    }

    fn get_custom_pool_length(&self) -> f64 {
        println!("Select custom pool length ( Between 13 and 200):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }

    fn get_single_or_repeat(&self) -> WorkoutStep {
        loop {
            println!("Single or Repeat?");
            println!("1) Single");
            println!("2) Repeat");
            let mut input = get_input("");
            match input{
                "1" => return ExecutableStepDTO,
                "2" => return RepeatGroupDTO,
                _ => continue,
            }
        }
    }

    fn get_step_type(&self) -> StepType {
        loop {
            println!("Enter step type:");
            println!("1) Warmup");
            println!("2) Cooldown");
            println!("3) Main");

            let mut input = get_input("");
            match input.trim() {
                "1" => return StepType{step_type_key: Step::Warmup},
                "2" => return StepType{step_type_key: Step::Cooldown},
                "3" => return StepType{step_type_key: Step::Main},
                _ => {
                    println!("Invalid choice, please try again");
                    continue;
                },
            }
        }
    }

    fn get_stroke_type(&self) -> StrokeType {
        loop {
            println!("Enter stroke type:");
            println!("1) Free");
            println!("2) Breast");
            println!("3) Back");
            println!("4) Butterfly");
            println!("5) IndividualMedley");

            let mut input = get_input("");
            let stroke = match input.trim() {
                "1" => Stroke::Free,
                "2" => Stroke::Breast,
                "3" => Stroke::Back,
                "4" => Stroke::Butterfly,
                "5" => Stroke::IndividualMedley,
                _ => continue,
            };

            break StrokeType{stroke_type_key: Some(stroke)}
        }
    }

    fn get_end_condition(&self) -> EndCondition{
        loop {
            println!("Enter end condition:");
            println!("1) LapButton");
            println!("2) Time");
            println!("3) Distance");
            println!("4) Iterations");

            let mut input = get_input("");

            match input.trim() {
                "1" => return EndCondition::new(Condition::LapButton),
                "2" => return EndCondition::new(Condition::Time),
                "3" => return EndCondition::new(Condition::Distance),
                "4" => return EndCondition::new(Condition::Iterations),
                _ => {
                    println!("Invalid choice, please try again");
                    continue
                }
            }
        }
    }

}
