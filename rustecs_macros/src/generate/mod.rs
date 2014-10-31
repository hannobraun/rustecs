use std::collections::HashMap;
use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ptr::P;

use parse;

use self::intermediate::Component;
use self::output::{
	Entities,
	Entity,
};


mod intermediate;
mod output;


type Components = HashMap<String, Component>;
type Items      = Vec<P<ast::Item>>;
type Tokens     = Vec<ast::TokenTree>;


pub fn items(context: &ExtCtxt, world: &parse::World) -> Items {
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

	let entities = Entities::generate(context, &components);
	let entity   = Entity::generate(context, &components);

	let mut items = Vec::new();
	items.push_all(vec![extern_crate_rustecs.unwrap()].as_slice());
	items.push_all(entities.0.as_slice());
	items.push_all(entity.0.as_slice());

	items
}
