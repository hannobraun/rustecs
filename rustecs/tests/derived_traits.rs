#![feature(phase)]


extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


world! {
	components Alpha;

	derived_traits Show;
}

pub type Alpha = u32;


#[test]
fn it_should_derive_the_traits_for_all_data_structures() {
	format!("{}", Entities::new());
	format!("{}", Entity::new());
	format!("{}", Systems::new());
}
