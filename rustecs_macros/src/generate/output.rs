use syntax::ext::base::ExtCtxt;

use super::{
	Components,
	Events,
	Items,
	Systems,
	Tokens,
};


pub struct EntitiesGenerator(pub Items);

impl EntitiesGenerator {
	pub fn generate(
		context   : &ExtCtxt,
		components: &Components,
		deriving  : &Tokens,
	) -> EntitiesGenerator {
		let collection_decls = EntitiesGenerator::collection_decls(components);
		let collection_inits = EntitiesGenerator::collection_inits(components);
		let inserts          = EntitiesGenerator::inserts(components);
		let removes          = EntitiesGenerator::removes(components);
		let field_sets       = EntitiesGenerator::field_sets(components);

		let structure = quote_item!(context,
			$deriving
			pub struct Entities {
				entities: ::std::collections::HashSet<_r::rustecs::EntityId>,
				next_id : _r::rustecs::EntityId,

				$collection_decls
			}
		);

		let implementation = quote_item!(context,
			impl Entities {
				pub fn new() -> Entities {
					Entities {
						entities: ::std::collections::HashSet::new(),
						next_id : 0,
						$collection_inits
					}
				}
			}
		);

		let trait_impl = quote_item!(context,
			impl _r::rustecs::EntityContainer<Entity> for Entities {
				fn add(&mut self, entity: Entity) -> _r::rustecs::EntityId {
					let id = self.next_id;
					self.next_id += 2;

					self.entities.insert(id);

					let world = self;
					$inserts

					id
				}

				fn import(&mut self, id: _r::rustecs::EntityId, entity: Entity) {
					self.entities.insert(id);

					let world = self;
					$inserts
				}

				fn remove(&mut self, id: _r::rustecs::EntityId) {
					self.entities.remove(&id);

					$removes
				}

				fn export(mut self) -> Vec<(_r::rustecs::EntityId, Entity)> {
					let ids: Vec<_r::rustecs::EntityId> = self.entities
						.iter()
						.map(|id| *id)
						.collect();

					ids
						.iter()
						.map(|id|
							(*id, Entity { $field_sets })
						)
						.collect()
				}
			}
		);

		let mut items = Vec::new();
		items.push(structure.unwrap());
		items.push(implementation.unwrap());
		items.push(trait_impl.unwrap());

		EntitiesGenerator(items)
	}

	fn collection_decls(components: &Components) -> Tokens {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.collection_decl.as_slice());
		}

		tokens
	}

	fn collection_inits(components: &Components) -> Tokens {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.collection_init.as_slice());
		}

		tokens
	}

	fn inserts(components: &Components) -> Tokens {
		let mut tokens = Vec::new();

		for (_, component) in components.iter() {
			tokens.push_all(component.insert.as_slice());
		}

		tokens
	}

	fn removes(components: &Components) -> Tokens {
		let mut removes = Vec::new();

		for (_, component) in components.iter() {
			removes.push_all(component.remove.as_slice());
		}

		removes
	}

	fn field_sets(components: &Components) -> Tokens {
		let mut init = Vec::new();

		for (_, component) in components.iter() {
			init.push_all(component.field_set.as_slice());
		}

		init
	}
}


pub struct EntityGenerator(pub Items);

impl EntityGenerator {
	pub fn generate(
		context   : &ExtCtxt,
		components: &Components,
		deriving  : &Tokens,
	) -> EntityGenerator {
		let field_decls = EntityGenerator::field_decls(components);
		let field_inits = EntityGenerator::field_inits(components);
		let builder_fns = EntityGenerator::builder_fns(components);

		let structure = quote_item!(context,
			$deriving
			pub struct Entity {
				$field_decls
			}
		);

		let implementation = quote_item!(context,
			impl Entity {
				pub fn new() -> Entity {
					Entity {
						$field_inits
					}
				}

				$builder_fns
			}
		);

		let mut items = Vec::new();
		items.push(structure.unwrap());
		items.push(implementation.unwrap());

		EntityGenerator(items)
	}

	fn field_decls(components: &Components) -> Tokens {
		let mut decls = Vec::new();

		for (_, component) in components.iter() {
			decls.push_all(component.field_decl.as_slice());
		}

		decls
	}

	fn field_inits(components: &Components) -> Tokens {
		let mut inits = Vec::new();

		for (_, component) in components.iter() {
			inits.push_all(component.field_init.as_slice());
		}

		inits
	}

	fn builder_fns(components: &Components) -> Tokens {
		let mut fns = Vec::new();

		for (_, component) in components.iter() {
			fns.push_all(component.builder_fn.as_slice());
		}

		fns
	}
}


pub struct EventGenerator(pub Items);

impl EventGenerator {
	pub fn generate(
		context : &ExtCtxt,
		events  : &Events,
		deriving: &Tokens,
	) -> EventGenerator {
		let mut variants: Tokens = Vec::new();
		for event in events.iter() {
			let name    = event.name;
			let variant = event.variant;

			variants.push_all(
				quote_tokens!(context,
					$variant($name),
				)
				.as_slice()
			);
		}

		let enumeration = quote_item!(context,
			$deriving
			pub enum Event {
				$variants
			}
		);

		EventGenerator(vec![
			enumeration.unwrap(),
		])
	}
}


pub struct SystemsGenerator(pub Items);

impl SystemsGenerator {
	pub fn generate(
		context : &ExtCtxt,
		events  : &Events,
		systems : Systems,
		deriving: &Tokens,
	) -> SystemsGenerator {
		let system_calls = SystemsGenerator::system_calls(
			context,
			events,
			systems,
		);

		let structure = quote_item!(context,
			$deriving
			pub struct Systems;
		);

		let implementation = quote_item!(context,
			impl Systems {
				pub fn new() -> Systems {
					Systems
				}

				pub fn trigger(&self, event: Event, _entities: &mut Entities) {
					match event {
						$system_calls
					}
				}
			}
		);

		SystemsGenerator(vec![
			structure.unwrap(),
			implementation.unwrap(),
		])
	}

	fn system_calls(
		context: &ExtCtxt,
		events : &Events,
		systems: Systems
	) -> Tokens {
		let mut tokens = Vec::new();

		for event in events.iter() {
			let mut calls_for_event = Vec::new();

			let name    = event.name;
			let variant = event.variant;

			let mut iter = systems
				.iter()
				.filter(
					|system|
						system.event == name
				);
			for system in iter {
				calls_for_event.push_all(system.call.as_slice());
			}


			tokens.push_all(
				quote_tokens!(context,
					$variant(mut _event) => {
						$calls_for_event
					},
				)
				.as_slice()
			);
		}

		tokens
	}
}
