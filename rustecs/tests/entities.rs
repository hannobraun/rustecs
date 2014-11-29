#![feature(phase)]


extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::EntityContainer;


world! {
	components Position<f64>, Score, state::WeaponState;
}


#[deriving(PartialEq, Show)]
pub struct Position<T>(T, T);

pub type Score = u32;

pub mod state {
	#[deriving(PartialEq, Show)]
	pub enum WeaponState {
		Reloading, Firing, Idle
	}
}


#[test]
fn it_should_create_entities() {
	use state::WeaponState;

	let mut entities = Entities::new();

	assert_eq!(0, entities.positions.len());
	assert_eq!(0, entities.scores.len());
	assert_eq!(0, entities.weapon_states.len());

	let missile_id = entities.add(
		Entity::new()
			.with_position(Position(8.0, 12.0))
	);

	assert_eq!(1, entities.positions.len());
	assert_eq!(0, entities.scores.len());
	assert_eq!(0, entities.weapon_states.len());

	assert_eq!(Position(8.0, 12.0), entities.positions[missile_id]);

	let ship_id = entities.add(
		Entity::new()
			.with_position(Position(0.0, 0.0))
			.with_score(100)
			.with_weapon_state(WeaponState::Idle)
	);

	assert_eq!(2, entities.positions.len());
	assert_eq!(1, entities.scores.len());
	assert_eq!(1, entities.weapon_states.len());

	assert_eq!(Position(0.0, 0.0), entities.positions[ship_id]);
	assert_eq!(100               , entities.scores[ship_id]);
	assert_eq!(WeaponState::Idle , entities.weapon_states[ship_id]);
}

#[test]
fn it_should_destroy_entities() {
	let mut entities = Entities::new();

	let id = entities.add(
		Entity::new()
			.with_position(Position(0.0, 0.0))
			.with_score(100)
	);

	entities.remove(id);

	assert_eq!(0, entities.positions.len());
	assert_eq!(0, entities.scores.len());
	assert_eq!(0, entities.weapon_states.len());
}
