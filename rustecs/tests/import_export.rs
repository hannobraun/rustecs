#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


world! {
	components Component;

	// Inline entity constructor. This is good for the general case, since it
	// avoids the duplication of external entity constructors.
	entity_constructor my_entity(value: u32) -> (Component) {
		(
			value,
		)
	}
}


pub type Component = u32;


#[test]
fn it_should_export_all_entities() {
	let mut world = World::new();

	let entity_1 = Entity {
		id       : 0, // ignored
		component: Some(5),
	};
	let entity_2 = Entity {
		id       : 1, //ignored
		component: Some(8),
	};

	let id_1 = world.create_entity(entity_1);
	let id_2 = world.create_entity(entity_2);

	let entities = world.export_entities();

	assert_eq!(2, entities.len());

	for &entity in entities.iter() {
		if entity.id == id_1 {
			assert_eq!(entity_1, entity);
		}
		else if entity.id == id_2 {
			assert_eq!(entity_2, entity);
		}
		else {
			fail!("Unexpected id: {}", entity.id);
		}
	}
}

#[test]
fn it_should_create_a_world_from_exported_entities() {
	let mut old_world = World::new();

	let id_1 = old_world.create_my_entity(5);
	let id_2 = old_world.create_my_entity(8);

	let world = World::from_entities(old_world.export_entities());

	assert_eq!(5, world.components[id_1]);
	assert_eq!(8, world.components[id_2]);
}

#[test]
fn it_should_import_entities() {
	let mut world = World::new();

	let entity = Entity {
		id       : 5,
		component: Some(8),
	};
	world.import_entity(entity);

	assert_eq!(1, world.components.len());
	assert_eq!(8, world.components[5]);
}
