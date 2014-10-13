use std::collections::HashMap;
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;
use syntax::ptr::P;

use names::{
	camel_to_snake_case,
	type_to_collection_name,
};
use parse;


type Components = HashMap<String, Component>;
type Tokens     = Vec<ast::TokenTree>;


pub fn items(context: &ExtCtxt, world: &parse::World) -> Vec<P<ast::Item>> {
	let extern_crate_rustecs = quote_item!(context,
		mod _r {
			extern crate rustecs;
		}
	);

	let components: Components = world.components
		.iter()
		.map(|&component|
			Component::generate(context, component)
		)
		.map(|component|
			(component.name.clone(), component)
		)
		.collect();

	let world  = World::generate(context, world.name, &components);
	let entity = Entity::generate(context, &components);

	let mut items = Vec::new();
	items.push_all(vec![extern_crate_rustecs.unwrap()].as_slice());
	items.push_all(world.0.as_slice());
	items.push_all(entity.0.as_slice());

	items
}


#[deriving(Clone, Show)]
pub struct Component {
	name    : String,
	var_name: ast::Ident,

	insert: Tokens,
	remove: Tokens,

	field_decl: Tokens,
	field_init: Tokens,
	field_set : Tokens,

	collection_decl: Tokens,
	collection_init: Tokens,

	builder_fn: Tokens,
}

impl Component {
	fn generate(context: &ExtCtxt, ty: ast::Ident) -> Component {
		let var_name = ast::Ident::new(
			token::intern(camel_to_snake_case(ty).as_slice())
		);
		let collection = ast::Ident::new(token::intern(
			type_to_collection_name(ty).as_slice()
		));
		let builder_name = {
			let mut builder_name = "with_".to_string();
			builder_name.push_str(var_name.as_str());

			ast::Ident::new(token::intern(
				builder_name.as_slice()
			))
		};

		let insert = quote_tokens!(context,
			match entity.$var_name {
				Some(component) => {
					let _ = world.$collection.insert(id, component);
				},
				None =>
					()
			}
		);
		let remove = quote_tokens!(context,
			self.$collection.remove(&id);
		);

		let field_decl = quote_tokens!(context,
			pub $var_name: Option<$ty>,
		);
		let field_init = quote_tokens!(context,
			$var_name: None,
		);
		let field_set = quote_tokens!(context,
			$var_name: self.$collection.find_copy(id),
		);

		let collection_decl = quote_tokens!(context,
			pub $collection: _r::rustecs::Components<$ty>,
		);
		let collection_init = quote_tokens!(context,
			$collection: _r::rustecs::components(),
		);

		let builder_fn = quote_tokens!(context,
			pub fn $builder_name(mut self, component: $ty) -> Entity {
				self.$var_name = Some(component);
				self
			}
		);

		Component {
			name    : token::get_ident(ty).to_string(),
			var_name: var_name,

			insert: insert,
			remove: remove,

			field_decl: field_decl,
			field_init: field_init,
			field_set : field_set,

			collection_decl: collection_decl,
			collection_init: collection_init,

			builder_fn: builder_fn,
		}
	}
}


struct World(Vec<P<ast::Item>>);

impl World {
	fn generate(
		context   : &ExtCtxt,
		name      : ast::Ident,
		components: &Components,
	) -> World {
		let collection_decls = World::collection_decls(components);
		let collection_inits = World::collection_inits(components);
		let inserts          = World::inserts(components);
		let removes          = World::removes(components);
		let field_sets       = World::field_sets(components);

		let structure = quote_item!(context,
			#[deriving(Show)]
			pub struct $name {
				entities: ::std::collections::HashSet<_r::rustecs::EntityId>,
				next_id : _r::rustecs::EntityId,

				$collection_decls
			}
		);

		let implementation = quote_item!(context,
			impl $name {
				pub fn new() -> $name {
					$name {
						entities: ::std::collections::HashSet::new(),
						next_id : 0,
						$collection_inits
					}
				}
			}
		);

		let trait_impl = quote_item!(context,
			impl _r::rustecs::Entities<Entity> for $name {
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

				fn export(&self) -> Vec<(_r::rustecs::EntityId, Entity)> {
					self.entities
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

		World(items)
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


struct Entity(Vec<P<ast::Item>>);

impl Entity {
	fn generate(context: &ExtCtxt, components: &Components) -> Entity {
		let field_decls = Entity::field_decls(components);
		let field_inits = Entity::field_inits(components);
		let builder_fns = Entity::builder_fns(components);

		let structure = quote_item!(context,
			#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
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

		Entity(items)
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
