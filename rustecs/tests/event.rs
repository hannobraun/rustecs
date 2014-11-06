#![feature(phase)]


extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


use rustecs::{
	Components,
	EntityContainer,
};


world! {
	components Component;

	events Init, Update;
}


pub struct Component;

pub struct Init;
pub struct Update;


#[test]
fn it_should_generate_an_event_enum() {
	let _init  : Event = InitEvent(Init);
	let _update: Event = UpdateEvent(Update);
}
