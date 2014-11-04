#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::EntityContainer;


world! {
	components Position, Score;
}


#[deriving(PartialEq, Show)]
pub struct Position(f64, f64);

pub type Score = u32;


#[test]
fn it_should_create_entities() {
	let mut entities = Entities::new();

	assert_eq!(0, entities.positions.len());
	assert_eq!(0, entities.scores.len());

	let missile_id = entities.add(
		Entity::new()
			.with_position(Position(8.0, 12.0))
	);

	assert_eq!(1, entities.positions.len());
	assert_eq!(0, entities.scores.len());

	assert_eq!(Position(8.0, 12.0), entities.positions[missile_id]);

	let ship_id = entities.add(
		Entity::new()
			.with_position(Position(0.0, 0.0))
			.with_score(100)
	);

	assert_eq!(2, entities.positions.len());
	assert_eq!(1, entities.scores.len());

	assert_eq!(Position(0.0, 0.0), entities.positions[ship_id]);
	assert_eq!(100               , entities.scores[ship_id]);
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
}
