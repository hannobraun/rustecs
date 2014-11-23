use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;

use names::{
	camel_to_snake_case,
	type_to_collection_name,
};
use parse;

use super::{
	Components,
	Tokens,
};


#[deriving(Clone, Show)]
pub struct Component {
	pub name    : String,
	pub var_name: ast::Ident,

	pub insert: Tokens,
	pub remove: Tokens,

	pub field_decl: Tokens,
	pub field_init: Tokens,
	pub field_set : Tokens,

	pub collection_decl: Tokens,
	pub collection_init: Tokens,
	pub collection_arg : Tokens,

	pub builder_fn: Tokens,
}

impl Component {
	pub fn generate(context: &ExtCtxt, ty: ast::Ident) -> Component {
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

        /* Todo: Parenthesis after $foo: are currently required in quote_tokens! to work around
         * ambiguity between macro-by-example $name:kind style matchers. Clean up once
         * rust-lang/rust#18775 is fixed.
         */
		let field_decl = quote_tokens!(context,
			pub $var_name: (Option<$ty>),
		);
		let field_init = quote_tokens!(context,
			$var_name: (None),
		);
		let field_set = quote_tokens!(context,
			$var_name: (self.$collection.pop(id)),
		);

		let collection_decl = quote_tokens!(context,
			pub $collection: (_r::rustecs::Components<$ty>),
		);
		let collection_init = quote_tokens!(context,
			$collection: (_r::rustecs::components()),
		);
		let collection_arg = quote_tokens!(context,
			&mut _entities.$collection,
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
			collection_arg : collection_arg,

			builder_fn: builder_fn,
		}
	}
}


pub struct Event {
	pub name: ast::Ident,
}

impl Event {
	pub fn generate(event: ast::Ident) -> Event {
		Event {
			name: event,
		}
	}
}


pub struct System {
	pub event: ast::Ident,
	pub call : Tokens,
}

impl System {
	pub fn generate(
		context   : &ExtCtxt,
		system    : &parse::System,
		components: &Components,
	) -> System {
		let name = system.name;

		let component_args =
			System::component_args(context, system, components);

		let call = quote_tokens!(context,
			$name(_event, $component_args);
		);

		System {
			event: system.event,
			call : call,
		}
	}

	fn component_args(
		context   : &ExtCtxt,
		system    : &parse::System,
		components: &Components,
	) -> Tokens {
		let mut tokens = Vec::new();

		for ident in system.components.iter() {
			let ref arg = components[ident.as_str().to_string()].collection_arg;

			tokens.push_all(
				quote_tokens!(context,
					$arg
				)
				.as_slice()
			);
		}

		tokens
	}
}
