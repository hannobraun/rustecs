use std::collections::HashMap;
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ptr::P;

use parse;

use self::intermediate::{
	Component,
	System,
};
use self::output::{
	EntitiesGenerator,
	EntityGenerator,
	SystemsGenerator,
};


mod intermediate;
mod output;


type Components = HashMap<String, Component>;
type Systems    = Vec<System>;

type Items      = Vec<P<ast::Item>>;
type Tokens     = Vec<ast::TokenTree>;


pub fn items(context: &ExtCtxt, world: &parse::World) -> Items {
	let extern_crate_rustecs = quote_item!(context,
		mod _r {
			extern crate rustecs;

			pub use std::any::Any;
			pub use std::intrinsics::TypeId;
		}
	);

	let mut derived_traits = Vec::new();
	for (i, ident) in world.derived_traits.iter().enumerate() {
		if i + 1 == world.derived_traits.len() {
			derived_traits.push_all(
				quote_tokens!(context,
					$ident
				)
				.as_slice()
			);
		}
		else {
			derived_traits.push_all(
				quote_tokens!(context,
					$ident,
				)
				.as_slice()
			);
		}
	}

	let deriving = if world.derived_traits.len() > 0 {
		quote_tokens!(context,
			#[deriving($derived_traits)]
		)
	}
	else {
		Vec::new()
	};

	let components: Components = world.components
		.iter()
		.map(|&component|
			Component::generate(context, component)
		)
		.map(|component|
			(component.name.clone(), component)
		)
		.collect();
	let systems: Systems = world.systems
		.iter()
		.map(|system|
			System::generate(context, system, &components)
		)
		.collect();

	let entities = EntitiesGenerator::generate(context, &components, &deriving);
	let entity   = EntityGenerator::generate(context, &components, &deriving);
	let systems  = SystemsGenerator::generate(
		context,
		world.events.as_slice(),
		systems,
		&deriving,
	);

	let mut items = Vec::new();
	items.push_all(vec![extern_crate_rustecs.unwrap()].as_slice());
	items.push_all(entities.0.as_slice());
	items.push_all(entity.0.as_slice());
	items.push_all(systems.0.as_slice());

	items
}
