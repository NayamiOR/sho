// use crate::entity::Entity;
// use crate::id::Id;
use sho_core::{entity::Entity, id::Id};

use std::collections::HashMap;

pub struct Base {
    entities: HashMap<Id, Entity>,
}
impl Base {
    // Main function for creating the base and read parsed script content
    fn load(entities: Vec<Entity>) -> Self {
        todo!()
    }

    fn add_entity(&mut self, entity: Entity) -> Result<Id, FailedToAddEntityError> {
        let id = Id::new();
        self.entities.insert(id, entity);
        Ok(id)
    }

    fn get_entity(&self, id: Id) -> Result<Entity, EntityNotFoundError> {
        self.entities.get(&id).map_or_else(
            || Err(EntityNotFoundError { id }),
            |entity| Ok(entity.clone()),
        )
    }

    fn find_by_label(&self, label: impl Into<String>) -> Option<Id> {
        todo!()
    }
}

struct EntityNotFoundError {
    id: Id,
}

struct FailedToAddEntityError {
    id: Id,
}
