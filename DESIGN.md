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


## Systems

There are a lot of different use cases for systems. Here are some design
considerations I can think of:
- When is the system called?
- Which entities are passed into a system?
- Which other arguments does the system need?


### When is the system called?

In the simplest case, all systems are called every frame, however, I've made the
experience that at some point you want something event-based, that allows to
execute a group of system when a specific event occurs.

It's possible to simulate such an event-based system by setting flags in
components and checking those flags in other systems. You can get pretty far
with this approach, but at some point it becomes unwieldy.

The syntax for event-triggered systems could look something like this (syntax
is based on the one proposed above in "Entity Constructors"):

``` Rust
world! {
	components A, B, C;

	event update;

	system on(update) for_components(A, B) = do_this_stuff;
	system on(update) for_components(B, C) = do_that_stuff;
}

fn do_this_stuff(a: A, b: B) {
	...
}
fn do_that_stuff(b: B, c: C) {
	...
}
```

The update event would be triggered by calling a method on `World`:

``` Rust
world.trigger_update();
```


### Which entities are passed into a system?

In my experience, most systems just want all entities with a given set of
components. That is pretty easy to realize. Either the system gets the
collections and iterates over them themselves, or the world iterates over them
and calls the system for each entity.

If the world does the iteration, the syntax could look like this (based on the
event-based approach above):

``` Rust
world! {
	components A, B, C;

	event update;

	system on(update) for_components(A, B, C) = do_stuff;
}

fn do_stuff(a: A, b: B, c: C) {
	...
}
```

I've also encountered cases, where a system will do something with a given
component if it's there but is okay with it not being there. This should also be
pretty straight-forward:

``` Rust
world! {
	components A, B, C;

	event update;

	system on(update) for_components(A, B) and_maybe(C) = do_stuff;
}

fn do_stuff(a: A, b: B, c: Option<C>) {
	...
}
```

The issue becomes less clear to me when it is combined with an event-based
approach. Not all events apply to all entities. For example, in my game, I
currently have code for the following situations:
- An entity is imported into the world (happens, if the client receives a new
  entity from the server).
- A client sends an action to the server.
- A ship launches a missiles.

I'm executing code in reaction to these events, but that code doesn't iterate
over all entities, it just needs the ones that are directly affected by the
event.

Looking at these use cases, I realize that for each of them the entities that
are affected are known when the event is triggered, so the code that does the
triggering could just pass them into the world:

``` Rust
world.trigger_user_action(affected_entities);
```

The world would then call the systems registered for that event for each
affected entity that has the components required by the system.

The issue becomes less clear when the affected entities are not yet known (for
example, if entities in given radius around the event location need to be
notified). Since I don't have a use case that requires that right now, I'd
rather defer this question for later.


### Which other arguments does the system need?

I think any other arguments that a system needs would be dependent on the event
that triggered the system. Here's the event example from above, extended with
arguments:

``` Rust
world! {
	components A, B, C;

	event update(delta_time_in_s: f64);

	system on(update) for_components(A, B, C) = do_stuff;
}

fn do_stuff(delta_time_in_s: f64, a: A, b: B, c: C) {
	...
}
```

The triggering of the event would look like this:

``` Rust
world.trigger_update(1.0 / 60.0);
```
