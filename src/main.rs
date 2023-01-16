use bevy::prelude::*;

fn main() {
    App::new().run();
}

// components
struct Position {
    x: f32,
    y: f32,
}

// systems
fn print_position_system(query: Query<&Transform>) {
    for t in query.iter() {
        println!("position: {:?}", t.translation);
    }
}
