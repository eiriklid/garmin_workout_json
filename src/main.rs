use crate::workout_builder::WorkoutBuilder;

mod garmin;
mod workout_builder;



fn main() {
    println!("Hello, world!");
    
    let builder = WorkoutBuilder{};
    builder.new_workout();


}
