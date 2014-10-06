#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


world! {
	components Component;
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

	for &(id, entity) in entities.iter() {
		if id == id_1 {
			assert_eq!(entity_1, entity);
		}
		else if id == id_2 {
			assert_eq!(entity_2, entity);
		}
		else {
			fail!("Unexpected id: {}", entity.id);
		}
	}
}

#[test]
fn it_should_import_entities() {
	let mut world = World::new();

	let entity = Entity {
		id       : 0, // ignored
		component: Some(8),
	};
	world.import_entity(5, entity);

	assert_eq!(1, world.components.len());
	assert_eq!(8, world.components[5]);
}
