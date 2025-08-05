use bevy::prelude::*;
use crate::components::*;


pub fn movement_system(
    mut query: Query<&mut Position, &Velocity>
) {
    for (mut pos, vel) in query.iter_mut() {
        pos.x += vel.x;
        pos.y += vel.y;
    }
}