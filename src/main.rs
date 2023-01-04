use bevy::prelude::*;
pub mod res;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ComplexResource>() //<ComplexResource>()
        .run();
}

/*
//.add_startup_system(add_people)
//.add_system(hello_world)
//.add_system(greet_people)
println!("hello world");
let a: i32 = 10;
println!("{a}");
// startup systems
//fn add_people(mut commands: Commands) {
//    commands.spawn((Person, Name("foo".to_string())));
//    commands.spawn((Person, Name("bar".to_string())));
//    commands.spawn((Person, Name("baz".to_string())));
//    commands.spawn((Person, Name("Steve Brown".to_string())));
//    commands.spawn(bundle: (Person, Name("Mary Sue".to_string())));
//    commands.sp
//}
// systems
fn hello_world() {
    println!("hello world!");
}
fn greet_people(query: Query<&Name, With<Person>>) {
    for n in query.iter() {
        println!("hello {}!", n.0)
    }
}
// components
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
 */
