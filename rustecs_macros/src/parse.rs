use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::parse;
use syntax::parse::parser::{ Parser, PathParsingMode };
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
	pub components    : Vec<ast::Path>,
	pub events        : Vec<ast::Ident>,
	pub systems       : Vec<System>,
	pub derived_traits: Vec<ast::Ident>,
}

impl World {
	fn parse(parser: &mut Parser) -> World {
		let mut components     = Vec::new();
		let mut events         = Vec::new();
		let mut systems        = Vec::new();
		let mut derived_traits = Vec::new();

		loop {
			let declaration = parser.parse_ident();
			match declaration.as_str() {
				"components" => {
					loop {
						components.push(parser.parse_path(PathParsingMode::LifetimeAndTypesWithoutColons).path);

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

				"system" => {
					systems.push(System::parse(parser));
				},

				"derived_traits" => {
					loop {
						derived_traits.push(parser.parse_ident());

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
			components    : components,
			events        : events,
			systems       : systems,
			derived_traits: derived_traits,
		}
	}
}


#[deriving(Show)]
pub struct System {
	pub name      : ast::Ident,
	pub event     : ast::Ident,
	pub components: Vec<ast::Ident>,
}

impl System {
	fn parse(parser: &mut Parser) -> System {
		let name = parser.parse_ident();

		let mut event     : Option<ast::Ident> = None;
		let mut components: Vec<ast::Ident>    = Vec::new();

		loop {
			let system_declaration = parser.parse_ident();
			match system_declaration.as_str() {
				"on" => {
					parser.expect(&token::OpenDelim(token::Paren));
					event = Some(parser.parse_ident());
					parser.expect(&token::CloseDelim(token::Paren));
				},

				"with" => {
					parser.expect(&token::OpenDelim(token::Paren));
					loop {
						components.push(parser.parse_ident());
						parser.eat(&token::Comma);
						if parser.eat(&token::CloseDelim(token::Paren)) {
							break;
						}
					}
				},

				_ =>
					parser.fatal(
						format!(
							"Expected 'on' or 'with', found{}",
							system_declaration.as_str(),
						)
						.as_slice()
					)
			}

			if parser.eat(&token::Semi) {
				break;
			}
		}

		let event = event.unwrap_or_else(|| {
			parser.fatal("You need to specify an event");
		});

		System {
			name      : name,
			event     : event,
			components: components,
		}
	}
}
