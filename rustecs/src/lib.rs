use std::collections::HashMap;


pub type EntityId      = u32;


pub type Components<T> = HashMap<EntityId, T>;

pub fn components<T>() -> Components<T> {
	HashMap::new()
}


pub trait Entities<E> {
	fn add(&mut self, entity: E) -> EntityId;
	fn import(&mut self, id: EntityId, entity: E);
	fn remove(&mut self, id: EntityId);

	fn export(&self) -> Vec<(EntityId, E)>;
}


pub struct Control<E> {
	pub added   : Vec<E>,
	pub imported: Vec<(EntityId, E)>,
	pub removed : Vec<EntityId>,
}

impl<E: Copy> Control<E> {
	pub fn new() -> Control<E> {
		Control {
			added   : Vec::new(),
			imported: Vec::new(),
			removed : Vec::new(),
		}
	}

	pub fn add(&mut self, entity: E) {
		self.added.push(entity);
	}

	pub fn import(&mut self, id: EntityId, entity: E) {
		self.imported.push((id, entity));
	}

	pub fn remove(&mut self, id: EntityId) {
		self.removed.push(id);
	}

	pub fn apply<Es: Entities<E>>(&mut self, entities: &mut Es) {
		for &entity in self.added.iter() {
			entities.add(entity);
		}
		for &(id, entity) in self.imported.iter() {
			entities.import(id, entity);
		}
		for &id in self.removed.iter() {
			entities.remove(id);
		}
	}
}
