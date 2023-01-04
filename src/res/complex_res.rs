use bevy::prelude::{Resource, FromWorld};

#[derive(Resource)]
struct ComplexResource {/* properties */}

impl FromWorld for ComplexResource {
    fn from_world(_w: &mut World) -> Self {
        // mutate other resources
        let mut _x = _w.get_resource_mut::<AlternateResource>().unwrap();

        ComplexResource {
            /* properties, logic */
        }
    }
}
