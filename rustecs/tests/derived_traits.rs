#![feature(phase)]


extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


world! {
	components Alpha;

	derived_traits Show;
}

pub type Alpha = u32;
