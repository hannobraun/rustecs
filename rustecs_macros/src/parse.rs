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
	pub components: Vec<ast::Ident>,
	pub events    : Vec<ast::Ident>,
}

impl World {
	fn parse(parser: &mut Parser) -> World {
		let mut components = Vec::new();
		let mut events     = Vec::new();

		loop {
			let declaration = parser.parse_ident();
			match declaration.as_str() {
				"components" => {
					loop {
						components.push(parser.parse_ident());

						parser.eat(&token::Comma);
						if parser.eat(&token::Semi) {
							break;
						}
					}
				},

				"events" => {
					loop {
						events.push(parser.parse_ident());

						parser.eat(&token::Comma);
						if parser.eat(&token::Semi) {
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

			if parser.eat(&token::Eof) {
				break;
			}
		}

		World {
			components: components,
			events    : events,
		}
	}
}
