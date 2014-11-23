#![feature(phase)]


extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::{
	Components,
	EntityContainer,
};


world! {
	components Alpha, Beta;

	events Init, Update;

	system init   on(Init)   with(Alpha, Beta);
	system update on(Update) with(Alpha, Beta);
}


pub type Alpha = bool;
pub type Beta  = bool;

pub struct Init;
pub struct Update;


fn init(
	_event: &mut Init,
	alphas: &mut Components<Alpha>,
	_     : &Components<Beta>
) {
	for (_, alpha) in alphas.iter_mut() {
		*alpha = true;
	}
}

fn update(
	_event: &mut Update,
	_     : &Components<Alpha>,
	betas : &mut Components<Beta>
) {
	for (_, beta) in betas.iter_mut() {
		*beta = true;
	}
}


#[test]
fn it_should_trigger_systems_by_event() {
	let mut entities = Entities::new();
	let     systems  = Systems::new();

	let id = entities.add(
		Entity::new()
			.with_alpha(false)
			.with_beta(false)
	);

	let mut update = Update;
	systems.trigger(Event::UpdateEvent(&mut update), &mut entities);

	assert_eq!(false, entities.alphas[id]);
	assert_eq!(true , entities.betas[id]);
}
