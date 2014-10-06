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


pub fn items(context: &ExtCtxt, world: &parse::World) -> Vec<P<ast::Item>> {
	let components: HashMap<String, Component> = world.components
		.iter()
		.map(|&component|
			Component::generate(context, component)
		)
		.map(|component|
			(component.name.clone(), component)
		)
		.collect();

	let world = World::generate(context, &components);

	let mut items = Vec::new();
	items.push_all(world.0.as_slice());

	items
}


#[deriving(Clone, Show)]
pub struct Component {
	name    : String,
	var_name: ast::Ident,
	import  : Vec<ast::TokenTree>,

	insert: Vec<ast::TokenTree>,
	remove: Vec<ast::TokenTree>,

	field_decl: Vec<ast::TokenTree>,
	field_set : Vec<ast::TokenTree>,

	collection_decl: Vec<ast::TokenTree>,
	collection_init: Vec<ast::TokenTree>,
}

impl Component {
	fn generate(context: &ExtCtxt, ty: ast::Ident) -> Component {
		let var_name = ast::Ident::new(
			token::intern(camel_to_snake_case(ty).as_slice()));
		let collection = ast::Ident::new(token::intern(
			type_to_collection_name(ty).as_slice()
		));

		let import = quote_tokens!(&*context,
			match entity.$var_name {
				Some(component) => {
					let _ = world.$collection.insert(id, component);
				},
				None =>
					()
			}
		);

		let insert = quote_tokens!(&*context,
			self.$collection.insert(id, $var_name);
		);
		let remove = quote_tokens!(&*context,
			self.$collection.remove(&id);
		);

		let field_decl = quote_tokens!(&*context,
			pub $var_name: Option<$ty>,
		);
		let field_set = quote_tokens!(&*context,
			$var_name: self.$collection.find_copy(id),
		);

		let collection_decl = quote_tokens!(&*context,
			pub $collection: ::rustecs::Components<$ty>,
		);
		let collection_init = quote_tokens!(&*context,
			$collection: ::rustecs::components(),
		);

		Component {
			name    : token::get_ident(ty).to_string(),
			var_name: var_name,
			import  : import,
			insert  : insert,
			remove  : remove,

			field_decl: field_decl,
			field_set : field_set,

			collection_decl: collection_decl,
			collection_init: collection_init,
		}
	}
}


struct World(Vec<P<ast::Item>>);

impl World {
	fn generate(
		context   : &ExtCtxt,
		components: &HashMap<String, Component>,
	) -> World {
		let collection_decls = World::collection_decls(components);
		let collection_inits = World::collection_inits(components);
		let imports          = World::imports(components);
		let removes          = World::removes(components);
		let field_decls      = World::field_decls(components);
		let field_sets       = World::field_sets(components);

		let structure = quote_item!(&*context,
			#[deriving(Show)]
			pub struct World {
				entities: ::std::collections::HashSet<::rustecs::EntityId>,
				next_id : ::rustecs::EntityId,

				$collection_decls
			}
		);

		let implementation = quote_item!(&*context,
			impl World {
				pub fn new() -> World {
					World {
						entities: ::std::collections::HashSet::new(),
						next_id : 0,
						$collection_inits
					}
				}

				pub fn export_entities(&self) -> Vec<(::rustecs::EntityId, Entity)> {
					self.entities
						.iter()
						.map(|id|
							(*id, Entity { $field_sets })
						)
						.collect()
				}

				pub fn import_entity(&mut self, id: ::rustecs::EntityId, entity: Entity) {
					self.entities.insert(id);
					let world = self;
					$imports
				}

				pub fn create_entity(&mut self, entity: Entity) -> ::rustecs::EntityId {
					let id = self.next_id;
					self.next_id += 1;

					self.entities.insert(id);

					let world = self;

					$imports

					id
				}

				pub fn destroy_entity(&mut self, id: ::rustecs::EntityId) {
					self.entities.remove(&id);

					$removes
				}
			}
		);

		let entity_struct = quote_item!(&*context,
			#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
			pub struct Entity {
				$field_decls
			}
		);

		let mut items = Vec::new();
		items.push(structure.unwrap());
		items.push(implementation.unwrap());
		items.push(entity_struct.unwrap());

		World(items)
	}

	fn collection_decls(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.collection_decl.as_slice());
		}

		tokens
	}

	fn collection_inits(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.collection_init.as_slice());
		}

		tokens
	}

	fn imports(components: &HashMap<String, Component>) -> Vec<ast::TokenTree> {
		let mut tokens = Vec::new();

		for (_, component) in components.iter() {
			tokens.push_all(component.import.as_slice());
		}

		tokens
	}

	fn removes(components: &HashMap<String, Component>) -> Vec<ast::TokenTree> {
		let mut removes = Vec::new();

		for (_, component) in components.iter() {
			removes.push_all(component.remove.as_slice());
		}

		removes
	}

	fn field_decls(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut decls = Vec::new();

		for (_, component) in components.iter() {
			decls.push_all(component.field_decl.as_slice());
		}

		decls
	}

	fn field_sets(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut init = Vec::new();

		for (_, component) in components.iter() {
			init.push_all(component.field_set.as_slice());
		}

		init
	}
}
