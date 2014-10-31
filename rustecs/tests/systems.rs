#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::{
	Components,
	EntityContainer,
};


world! {
	components Alpha, Beta;

	events Update;

	system update on(Update) with(Alpha, Beta);
}


pub type Alpha = bool;
pub type Beta  = bool;

pub struct Update;

fn update(
	_alphas: &Components<Alpha>,
	betas  : &mut Components<Beta>
) {
	for (_, beta) in betas.iter_mut() {
		*beta = true;
	}
}


#[test]
fn it_should_pass_components_into_a_system() {
	let mut entities = Entities::new();
	let     systems  = Systems::new();

	let id = entities.add(
		Entity::new()
			.with_alpha(false)
			.with_beta(false)
	);

	systems.trigger(Update, &mut entities);

	assert_eq!(true, entities.betas[id]);
}
