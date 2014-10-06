#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


world! {
	components Position, Score;
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Position(f64, f64);

pub type Score = u32;


#[test]
fn it_should_create_entities() {
	let mut world = World::new();

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.scores.len());

	let missile = Entity::new()
		.with_position(Position(8.0, 12.0));
	let missile_id = world.create_entity(missile);

	assert_eq!(1, world.positions.len());
	assert_eq!(0, world.scores.len());

	assert_eq!(Position(8.0, 12.0), world.positions[missile_id]);

	let ship = Entity::new()
		.with_position(Position(0.0, 0.0))
		.with_score(100);
	let ship_id = world.create_entity(ship);

	assert_eq!(2, world.positions.len());
	assert_eq!(1, world.scores.len());

	assert_eq!(Position(0.0, 0.0), world.positions[ship_id]);
	assert_eq!(100               , world.scores[ship_id]);
}

#[test]
fn it_should_destroy_entities() {
	let mut world = World::new();

	let ship = Entity::new()
		.with_position(Position(0.0, 0.0))
		.with_score(100);
	let id = world.create_entity(ship);

	world.destroy_entity(id);

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.scores.len());
}
