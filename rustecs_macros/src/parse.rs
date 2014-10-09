use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse;
use syntax::parse::parser::Parser;
use syntax::parse::token;


pub fn parse(context: &ExtCtxt, token_tree: &[ast::TokenTree]) -> World {
	let mut parser = parse::new_parser_from_tts(
		context.parse_sess(),
		context.cfg(),
		token_tree.to_vec()
	);

	World::parse(&mut parser)
}


#[deriving(Show)]
pub struct World {
	pub name      : ast::Ident,
	pub components: Vec<ast::Ident>,
}

impl World {
	fn parse(parser: &mut Parser) -> World {
		let mut components = Vec::new();

		let name = parser.parse_ident();
		parser.expect(&token::COMMA);

		loop {
			let declaration = parser.parse_ident();
			match declaration.as_str() {
				"components" => {
					loop {
						components.push(parser.parse_ident());

						parser.eat(&token::COMMA);
						if parser.eat(&token::SEMI) {
							break;
						}
					}
				},

				_ =>
					parser.fatal(
						format!(
							"Unexpected declaration: {}",
							declaration.as_str(),
						)
						.as_slice()
					)
			}

			if parser.eat(&token::EOF) {
				break;
			}
		}

		World {
			name      : name,
			components: components,
		}
	}
}
