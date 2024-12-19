use bevy::prelude::Entity;

use super::single_batch::convert_collidables_to_wgsl_types::PerCollidableDataRequiredByGpu;

#[derive(Debug, Clone, PartialEq)]
pub struct CollidableMetadata {
    pub entity: Entity,
    pub is_sensor: bool,
    pub x: f32,
    pub y: f32,
}

impl From<&PerCollidableDataRequiredByGpu> for CollidableMetadata {
    fn from(collidable: &PerCollidableDataRequiredByGpu) -> Self {
        Self {
            entity: collidable.entity,
            is_sensor: collidable.is_sensor,
            x: collidable.center_x,
            y: collidable.center_y,
        }
    }
}

impl Eq for CollidableMetadata {}

// impl std::hash::Hash for CollidableMetadata {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.entity.hash(state);
//         self.is_sensor.hash(state);
//     }
// }
