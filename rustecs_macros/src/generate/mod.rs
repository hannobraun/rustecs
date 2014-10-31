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

	let entities = EntitiesGenerator::generate(context, &components);
	let entity   = EntityGenerator::generate(context, &components);
	let systems  = SystemsGenerator::generate(
		context,
		world.events.as_slice(),
		systems
	);

	let mut items = Vec::new();
	items.push_all(vec![extern_crate_rustecs.unwrap()].as_slice());
	items.push_all(entities.0.as_slice());
	items.push_all(entity.0.as_slice());
	items.push_all(systems.0.as_slice());

	items
}
