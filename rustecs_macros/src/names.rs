use syntax::ast;
use syntax::parse::token;


pub fn camel_to_snake_case(ident: ast::Ident) -> String {
	let camel_case = token::get_ident(ident).to_string().into_ascii();

	let mut snake_case = String::new();
	for (i, c) in camel_case.iter().enumerate() {
		if c.is_uppercase() && i != 0 {
			snake_case.push_str("_");
		}

		snake_case.push(c.to_lowercase().as_char());
	}

	snake_case
}

pub fn type_to_collection_name(ident: ast::Ident) -> String {
	pluralize(camel_to_snake_case(ident))
}

fn pluralize(s: String) -> String {
	let mut p = s.clone();

	if s.as_slice().ends_with("y") {
		p.pop();
		p.push_str("ies");
		p
	}
	else {
		p.push_str("s");
		p
	}
}
