#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


world! {
	// Inline entity constructor. This is good for the general case, since it
	// avoids the duplication of external entity constructors.
	entity_constructor missile(x: f64, y: f64) -> (Position, Visual) {
		(
			Position(x, y),
			RenderAsMissile,
		)
	}

	// This specifies an entity constructor that uses an external function. Can
	// be useful for debugging, since compiler errors inside generated code are
	// not very useful. There's a lot of duplication between the declaration
	// here and the external function though.
	entity_constructor ship(score: u32) -> (Position, Visual, Score) = create_ship;
}


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub struct Position(f64, f64);

#[deriving(Clone, Decodable, Encodable, Eq, PartialEq, Show)]
pub enum Visual {
	RenderAsMissile,
	RenderAsShip,
}

pub type Score = u32;


fn create_ship(score: u32) -> (Position, Visual, Score) {
	(
		Position(0.0, 0.0),
		RenderAsShip,
		score,
	)
}


#[test]
fn it_should_create_entities() {
	let mut world = World::new();

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.visuals.len());
	assert_eq!(0, world.scores.len());

	let missile_id = world.create_missile(8.0, 12.0);

	assert_eq!(1, world.positions.len());
	assert_eq!(1, world.visuals.len());
	assert_eq!(0, world.scores.len());

	assert_eq!(Position(8.0, 12.0), world.positions[missile_id]);
	assert_eq!(RenderAsMissile    , world.visuals[missile_id]);

	let ship_id = world.create_ship(100);

	assert_eq!(2, world.positions.len());
	assert_eq!(2, world.visuals.len());
	assert_eq!(1, world.scores.len());

	assert_eq!(Position(0.0, 0.0), world.positions[ship_id]);
	assert_eq!(RenderAsShip      , world.visuals[ship_id]);
	assert_eq!(100               , world.scores[ship_id]);
}

#[test]
fn it_should_destroy_entities() {
	let mut world = World::new();

	let id = world.create_ship(100);
	world.destroy_entity(id);

	assert_eq!(0, world.positions.len());
	assert_eq!(0, world.visuals.len());
	assert_eq!(0, world.scores.len());
}

#[test]
fn it_should_export_all_entities() {
	let mut world = World::new();

	let missile_id = world.create_missile(8.0, 12.0);
	let ship_id    = world.create_ship(100);

	let entities = world.export_entities();

	assert_eq!(2, entities.len());

	let missile = Entity {
		id      : missile_id,
		position: Some(Position(8.0, 12.0)),
		visual  : Some(RenderAsMissile),
		score   : None
	};
	let ship = Entity {
		id      : ship_id,
		position: Some(Position(0.0, 0.0)),
		visual  : Some(RenderAsShip),
		score   : Some(100),
	};

	for &entity in entities.iter() {
		if entity.id == missile_id {
			assert_eq!(missile, entity);
		}
		else if entity.id == ship_id {
			assert_eq!(ship, entity);
		}
		else {
			fail!("Unexpected id: {}", entity.id);
		}
	}
}

#[test]
fn it_should_create_a_world_from_exported_entities() {
	let mut old_world = World::new();

	let missile_id = old_world.create_missile(8.0, 12.0);
	let ship_id    = old_world.create_ship(100);

	let world = World::from_entities(old_world.export_entities());

	assert_eq!(2, world.positions.len());
	assert_eq!(2, world.visuals.len());
	assert_eq!(1, world.scores.len());

	assert_eq!(Position(8.0, 12.0), world.positions[missile_id]);
	assert_eq!(RenderAsMissile    , world.visuals[missile_id]);

	assert_eq!(Position(0.0, 0.0), world.positions[ship_id]);
	assert_eq!(RenderAsShip      , world.visuals[ship_id]);
	assert_eq!(100               , world.scores[ship_id]);
}

#[test]
fn it_should_import_entities() {
	let mut world = World::new();

	let entity = Entity {
		id      : 5,
		position: Some(Position(8.0, 12.0)),
		visual  : Some(RenderAsMissile),
		score   : None,
	};
	world.import_entity(entity);

	let entities = world.export_entities();

	assert_eq!(1, entities.len());
	assert_eq!(entity, entities[0]);
}
