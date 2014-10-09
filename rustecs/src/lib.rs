use std::collections::HashMap;


pub type EntityId      = u32;


pub type Components<T> = HashMap<EntityId, T>;

pub fn components<T>() -> Components<T> {
	HashMap::new()
}


pub trait Entities<E> {
	fn new() -> Self;

	fn add(&mut self, entity: E) -> EntityId;
	fn import(&mut self, id: EntityId, entity: E);
	fn remove(&mut self, id: EntityId);

	fn export(&self) -> Vec<(EntityId, E)>;
}
