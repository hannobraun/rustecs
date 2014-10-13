#![feature(phase)]


extern crate serialize;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::{
	Control,
	Entities,
};


world! { MyEntities,
	components Component;
}

pub type Component = u16;


#[test]
fn it_should_add_entities_after_apply() {
	let mut entities = MyEntities::new();
	let mut control  = Control::new();

	control.add(Entity::new().with_component(5));

	assert_eq!(0, entities.components.len());

	control.apply(&mut entities);

	assert_eq!(1, entities.components.len());
}
