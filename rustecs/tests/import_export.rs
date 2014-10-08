#![feature(phase)]


extern crate serialize;

#[phase(plugin)] extern crate rustecs_macros;


world! {
	components Component;
}


pub type Component = u32;


#[test]
fn it_should_export_all_entities() {
	let mut world = World::new();

	let entity_1 = Entity::new().with_component(5);
	let entity_2 = Entity::new().with_component(8);

	let id_1 = world.add_entity(entity_1);
	let id_2 = world.add_entity(entity_2);

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
			fail!("Unexpected id: {}", id);
		}
	}
}

#[test]
fn it_should_import_entities() {
	let mut world = World::new();

	let id = 5;
	world.import_entity(id, Entity::new().with_component(8));

	assert_eq!(1, world.components.len());
	assert_eq!(8, world.components[id]);
}
