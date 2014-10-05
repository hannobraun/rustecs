# Design

This document contains a few notes regarding the design of Rustecs and future
design directions. I wrote this mostly for myself, to have a place to jot down
ideas (writing them down helps my thought process a lot). However, feel free to
form your own opinions and [tell me about them](mailto:mail@hannobraun.de).


## Entity Constructors

Rustecs doesn't really care about the "type" of an entity. The type is solely
defined by the components it has. There's a single inconsistency however,
`entity_constructor`.

One scenario where this inconsistency shows: What if I have a component that no
entity is ever constructed with, that is only added had runtime? The only way to
define such a component would be by including it in a fake `entity_constructor`
that is never used.

Entity constructors are definitely useful, but I'm not sure they belong in
`World`. Maybe they could be the responsibility of the user without much loss in
ergonomics. In that scenario, components would be specified directly.

A minimal world definition would look like this:
``` Rust
world! {
	components A, B, C;
}
```

There already is an `Entity` struct used for importing/exporting. It could be
used for creation too:

``` Rust
world.create_entity(Entity { a: Some(a), b: Some(b), c: None });
```

`Entity` could get some builder methods to make this more friendly:

``` Rust
world.create_entity(
	Entity::new()
		.with_a(a)
		.with_b(b)
);
```

This could already be good enough for a lot of cases. When it is not enough, the
user can just define their own entity constructor:

``` Rust
world.create_entity(
	my_entity_constructor(my_args)
);
```
