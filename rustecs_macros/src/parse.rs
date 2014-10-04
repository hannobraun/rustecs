use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse;
use syntax::parse::common::seq_sep_trailing_allowed;
use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ptr::P;


pub fn parse(context: &ExtCtxt, token_tree: &[ast::TokenTree]) -> Vec<Entity> {
	let mut parser = parse::new_parser_from_tts(
		context.parse_sess(),
		context.cfg(),
		token_tree.to_vec()
	);

	let mut entities = Vec::new();

	loop {
		entities.push(Entity::parse(&mut parser));

		if parser.eat(&token::EOF) {
			break;
		}
	}

	entities
}


pub struct Entity {
	pub name       : ast::Ident,
	pub components : Vec<ast::Ident>,
	pub args       : Vec<ast::Arg>,
	pub constr_impl: ConstructorImpl,
}

impl Entity {
	fn parse(parser: &mut Parser) -> Entity {
		let declaration_type = parser.parse_ident();
		if declaration_type.as_str() != "entity_constructor" {
			parser.fatal(
				format!(
					"Expected entity_constructor, found {}",
					declaration_type.as_str(),
				)
				.as_slice()
			);
		}

		let name = parser.parse_ident();

		let args = parser.parse_unspanned_seq(
			&token::LPAREN,
			&token::RPAREN,
			seq_sep_trailing_allowed(token::COMMA),
			|p| p.parse_arg());

		parser.expect(&token::RARROW);

		let components = parser.parse_unspanned_seq(
			&token::LPAREN,
			&token::RPAREN,
			seq_sep_trailing_allowed(token::COMMA),
			|p| p.parse_ident());


		let constructor_impl = if parser.eat(&token::EQ) {
			let constructor_fn_name = parser.parse_ident();
			parser.expect(&token::SEMI);

			External(constructor_fn_name)
		}
		else {
			Inline(parser.parse_block())
		};

		Entity {
			name       : name,
			components : components,
			args       : args,
			constr_impl: constructor_impl,
		}
	}
}

pub enum ConstructorImpl {
	Inline(P<ast::Block>),
	External(ast::Ident),
}
