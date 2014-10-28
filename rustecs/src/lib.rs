use std::collections::HashMap;


pub type EntityId = u32;


pub type Components<T> = HashMap<EntityId, T>;

pub fn components<T>() -> Components<T> {
	HashMap::new()
}


pub trait EntityContainer<E> {
	fn add(&mut self, entity: E) -> EntityId;
	fn import(&mut self, id: EntityId, entity: E);
	fn remove(&mut self, id: EntityId);

	fn export(&self) -> Vec<(EntityId, E)>;
}


pub struct Control<E> {
	next_id : EntityId,
	imported: Vec<(EntityId, E)>,
	removed : Vec<EntityId>,
}

impl<E: Clone> Control<E> {
	pub fn new() -> Control<E> {
		Control {
			next_id : 1, // generate odd ids to avoid collisions
			imported: Vec::new(),
			removed : Vec::new(),
		}
	}

	pub fn add(&mut self, entity: E) -> EntityId {
		let id = self.next_id;
		self.next_id += 2;

		self.imported.push((id, entity));
		id
	}

	pub fn import(&mut self, id: EntityId, entity: E) {
		self.imported.push((id, entity));
	}

	pub fn remove(&mut self, id: EntityId) {
		self.removed.push(id);
	}

	pub fn apply<Es: EntityContainer<E>>(&mut self, entities: &mut Es) {
		for &(id, ref entity) in self.imported.iter() {
			entities.import(id, entity.clone());
		}
		for &id in self.removed.iter() {
			entities.remove(id);
		}

		self.imported.clear();
		self.removed.clear();
	}
}
