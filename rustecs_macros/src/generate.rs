use std::collections::HashMap;
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;
use syntax::ptr::P;

use names::{
	camel_to_snake_case,
	type_to_collection_name,
};
use parse::{
	mod,
	External,
	Inline
};


pub fn items(context: &ExtCtxt, ecs: &Vec<parse::EntityConstructor>) -> Vec<P<ast::Item>> {
	let components = Component::generate_components(context, ecs);

	let entities: Vec<Entity> = ecs
		.iter()
		.map(|entity|
			Entity::generate(context, entity, &components))
		.collect();

	let world = World::generate(context, &entities);

	let mut items = Vec::new();
	items.push_all(world.0.as_slice());

	items
}


#[deriving(Clone, Show)]
pub struct Component {
	name: String,

	var_name   : ast::Ident,
	decl       : Vec<ast::TokenTree>,
	init       : Vec<ast::TokenTree>,
	import     : Vec<ast::TokenTree>,
	insert     : Vec<ast::TokenTree>,
	remove     : Vec<ast::TokenTree>,
	entity_decl: Vec<ast::TokenTree>,
	entity_init: Vec<ast::TokenTree>,
}

impl Component {
	fn generate_components(
		context : &ExtCtxt,
		entities: &Vec<parse::EntityConstructor>
	) -> HashMap<String, Component> {
		let mut components = HashMap::new();

		for entity in entities.iter() {
			for &name in entity.components.iter() {
				let component = Component::generate(context, name);
				components.insert(
					component.name.clone(),
					component,
				);
			}
		}

		components
	}

	fn generate(context: &ExtCtxt, ty: ast::Ident) -> Component {
		let var_name = ast::Ident::new(
			token::intern(camel_to_snake_case(ty).as_slice()));

		let collection = ast::Ident::new(token::intern(
			type_to_collection_name(ty).as_slice()
		));

		let decl = quote_tokens!(&*context,
			pub $collection: ::rustecs::Components<$ty>,
		);

		let init = quote_tokens!(&*context,
			$collection: ::rustecs::components(),
		);

		let import = quote_tokens!(&*context,
			match entity.$var_name {
				Some(component) => {
					let _ = world.$collection.insert(entity.id, component);
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

		let entity_decl = quote_tokens!(&*context,
			pub $var_name: Option<$ty>,
		);

		let entity_init = quote_tokens!(&*context,
			$var_name: self.$collection.find_copy(id),
		);

		Component {
			name       : token::get_ident(ty).to_string(),
			var_name   : var_name,
			decl       : decl,
			init       : init,
			import     : import,
			insert     : insert,
			remove     : remove,
			entity_decl: entity_decl,
			entity_init: entity_init,
		}
	}
}


#[deriving(Clone)]
pub struct Entity {
	name      : ast::Ident,
	components: HashMap<String, Component>,
	create_fn : Vec<ast::TokenTree>,
}

impl Entity {
	fn generate(
		context       : &ExtCtxt,
		entity        : &parse::EntityConstructor,
		all_components: &HashMap<String, Component>
	) -> Entity {
		let entity_components = entity.components
			.iter()
			.map(|&ident| {
				let name = token::get_ident(ident).to_string();
				(name.clone(), all_components[name].clone())
			})
			.collect();

		let ordered_components: Vec<String> = entity.components
			.iter()
			.map(|&ident|
				token::get_ident(ident).to_string())
			.collect();

		let create_fn = Entity::create_fn(
			context,
			entity,
			&entity_components,
			&ordered_components);

		Entity {
			name      : entity.name,
			components: entity_components,
			create_fn : create_fn,
		}
	}

	fn create_fn(
		context           : &ExtCtxt,
		entity            : &parse::EntityConstructor,
		components        : &HashMap<String, Component>,
		ordered_components: &Vec<String>
	) -> Vec<ast::TokenTree> {
		let mut name = "create_".to_string();
		name.push_str(camel_to_snake_case(entity.name).as_slice());
		let name = ast::Ident::new(token::intern(name.as_slice()));

		let mut args = Vec::new();
		for arg in entity.args.iter() {
			args.push_all(quote_tokens!(&*context, $arg,).as_slice());
		}

		let mut component_names = Vec::new();
		for name in ordered_components.iter() {
			let ref component = components[name.clone()];
			let var_name  = component.var_name;

			component_names.push_all(
				quote_tokens!(&*context, $var_name,).as_slice());
		}

		let constr_call = match entity.constr_impl {
			Inline(ref block) => {
				quote_tokens!(context, $block)
			},
			External(fn_name) => {
				let mut constr_args = Vec::new();
				for arg in entity.args.iter() {
					let arg_name = match arg.pat.node {
						ast::PatIdent(_, spanned, _) => {
							let arg_name = spanned.node;
							constr_args.push_all(
								quote_tokens!(context,
									$arg_name,
								)
								.as_slice()
							);
						},

						_ => fail!("Expected an identifier"),
					};
				}

				quote_tokens!(context, $fn_name($constr_args))
			},
		};

		let mut inserts = Vec::new();
		for (_, component) in components.iter() {
			inserts.push_all(component.insert.as_slice());
		}

		quote_tokens!(&*context,
			pub fn $name(&mut self, $args) -> ::rustecs::EntityId {
				let id = self.next_id;
				self.next_id += 1;

				let ($component_names) = $constr_call;

				self.entities.insert(id);
				$inserts

				id
			}
		)
	}
}


struct World(Vec<P<ast::Item>>);

impl World {
	fn generate(context: &ExtCtxt, entities: &Vec<Entity>) -> World {
		let components = World::components(entities);

		let decls        = World::component_decls(&components);
		let inits        = World::component_inits(&components);
		let imports      = World::imports(&components);
		let create_fns   = World::create_fns(entities);
		let removes      = World::removes(&components);
		let entity_decls = World::entity_decls(&components);
		let entity_init  = World::entity_init(&components);

		let structure = quote_item!(&*context,
			#[deriving(Show)]
			pub struct World {
				entities: ::std::collections::HashSet<::rustecs::EntityId>,
				next_id : ::rustecs::EntityId,

				$decls
			}
		);

		let implementation = quote_item!(&*context,
			impl World {
				pub fn new() -> World {
					World {
						entities: ::std::collections::HashSet::new(),
						next_id : 0,
						$inits
					}
				}

				pub fn from_entities(entities: Vec<Entity>) -> World {
					let mut world = World {
						entities: ::std::collections::HashSet::new(),
						next_id : 0,
						$inits
					};

					for entity in entities.move_iter() {
						world.entities.insert(entity.id);
						if entity.id > world.next_id {
							world.next_id = entity.id + 1;
						}

						$imports
					}

					world
				}

				pub fn export_entities(&self) -> Vec<Entity> {
					self.entities
						.iter()
						.map(|id|
							Entity {
								id: *id,
								$entity_init
							})
						.collect()
				}

				pub fn import_entity(&mut self, entity: Entity) {
					self.entities.insert(entity.id);
					let world = self;
					$imports
				}

				$create_fns

				pub fn destroy_entity(&mut self, id: ::rustecs::EntityId) {
					self.entities.remove(&id);

					$removes
				}
			}
		);

		let entity_struct = quote_item!(&*context,
			#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
			pub struct Entity {
				pub id: ::rustecs::EntityId,
				$entity_decls
			}
		);

		let mut items = Vec::new();
		items.push(structure.unwrap());
		items.push(implementation.unwrap());
		items.push(entity_struct.unwrap());

		World(items)
	}

	fn components(entities: &Vec<Entity>) -> HashMap<String, Component> {
		let mut components = HashMap::new();

		for entity in entities.iter() {
			for (name, component) in entity.components.iter() {
				components.insert((*name).clone(), (*component).clone());
			}
		}

		components
	}

	fn component_decls(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.decl.as_slice());
		}

		tokens
	}

	fn component_inits(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut tokens = vec!();

		for (_, component) in components.iter() {
			tokens.push_all(component.init.as_slice());
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

	fn create_fns(entities: &Vec<Entity>) -> Vec<ast::TokenTree> {
		let mut tokens = Vec::new();

		for entity in entities.iter() {
			tokens.push_all(entity.create_fn.as_slice());
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

	fn entity_decls(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut decls = Vec::new();

		for (_, component) in components.iter() {
			decls.push_all(component.entity_decl.as_slice());
		}

		decls
	}

	fn entity_init(
		components: &HashMap<String, Component>
	) -> Vec<ast::TokenTree> {
		let mut init = Vec::new();

		for (_, component) in components.iter() {
			init.push_all(component.entity_init.as_slice());
		}

		init
	}
}
