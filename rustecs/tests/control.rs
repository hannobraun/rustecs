#![feature(phase)]


extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::{
	Control,
	EntityContainer,
};


world! {
	components Component;
}

pub type Component = u16;


#[test]
fn it_should_add_entities_after_apply() {
	let mut entities = Entities::new();
	let mut control  = Control::new();

	control.add(Entity::new().with_component(5));

	assert_eq!(0, entities.components.len());

	control.apply(&mut entities);

	assert_eq!(1, entities.components.len());
}

#[test]
fn it_should_import_entities_after_apply() {
	let mut entities = Entities::new();
	let mut control  = Control::new();

	control.import(3, Entity::new().with_component(5));

	assert_eq!(0, entities.components.len());

	control.apply(&mut entities);

	assert_eq!(1, entities.components.len());
	assert_eq!(5, entities.components[3]);
}

#[test]
fn it_should_remove_entities_after_apply() {
	let mut entities = Entities::new();
	let mut control  = Control::new();

	let id = entities.add(Entity::new().with_component(5));

	control.remove(id);

	assert_eq!(1, entities.components.len());

	control.apply(&mut entities);

	assert_eq!(0, entities.components.len());
}

#[test]
fn it_should_return_a_unique_id_from_add() {
	let mut entities = Entities::new();
	let mut control  = Control::new();

	entities.add(Entity::new().with_component(3));
	entities.add(Entity::new().with_component(5));

	let id = control.add(Entity::new().with_component(8));
	control.apply(&mut entities);

	assert_eq!(3, entities.components.len());
	assert_eq!(8, entities.components[id]);
}

#[test]
fn it_should_apply_adds_only_once() {
	let mut entities = Entities::new();
	let mut control  = Control::new();

	let id = control.add(Entity::new().with_component(5));

	control.apply(&mut entities);
	entities.remove(id);
	control.apply(&mut entities);

	assert_eq!(0, entities.components.len());
}

#[test]
fn it_should_apply_removes_only_once() {
	let mut entities = Entities::new();
	let mut control  = Control::new();

	let id = entities.add(Entity::new().with_component(5));

	control.remove(id);
	control.apply(&mut entities);
	entities.import(id, Entity::new().with_component(5));
	control.apply(&mut entities);

	assert_eq!(1, entities.components.len());
}
