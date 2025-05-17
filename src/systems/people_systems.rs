use crate::components::{greet_timer::GreetTimer, name::Name, person::Person};
use bevy::prelude::*;

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ross".to_string())));
    commands.spawn((Person, Name("Chris".to_string())));
    commands.spawn((Person, Name("Astro".to_string())));
}

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Astro" {
            name.0 = "Astro Nova".to_string();
            break;
        }
    }
}
