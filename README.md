# Rustecs

## About

Rustecs is an [Entity/Component System](http://entity-systems.wikidot.com/)
written in [Rust](http://rust-lang.org). It's being used to develop
[Von Neumann Defense Force](http://vndf.de).

Please note that while Rustecs works and is usable, it comes with one caveat.
I'm currently developing it for the needs of one specific game, which brings a
few limitations:
- It's usable but incomplete. I only implement features I actually need and know
  how to design. Some features you would expect from a generic entity system
  aren't there yet. For some missing features I use crazy workarounds in my own
  code because I'm not sure yet how to design a general API for them yet.
- Performance is good enough but probably not better. I usually go with the
  simplest solution for a given problem without worrying about performance too
  much. So far that has worked well for me, but it might not be enough for you.

If you tried Rustecs and find it lacking, feel free to send me pull request to
address your concerns!


## Documentation

### Introduction

The documentation in this README is intended to explain the general concepts of
Rustecs and how they should be used. To see an actual example, please take a
look at the unit tests:
https://github.com/hannobraun/rustecs/blob/master/rustecs/tests/


### The Basics

Rustecs is implemented as a compiler plugin that generates the code for your
entity system from specifications in a simple
[DSL](http://en.wikipedia.org/wiki/Domain-specific_language).

Let's imagine a simple Asteroids-like game. Here's what the definition for the
Entity/Component System could look like:

``` Rust
world! {
	entity_constructor asteroid(x: i16, y: i16, size: u16) -> (Position, Size) {
		(
			Position { x: x, y: y },
			Size(size),
		)
	}

	entity_constructor ship() -> (Position, Score) {
		(
			Position { x: 0, y: 0 },
			0,
		)
	}
}
```

This defines a world with two kinds of entities (`asteroid` and `ship`) and
three kinds of components (`Position`, `Size` and `Score`).


### Components

The example is not complete though since we haven't defined what the components
actually are. Components are just pieces of data and they can be any of Rust's
data types.

Here are the component definitions that complete the example:

``` Rust
// Regular struct
#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
struct Position {
	x: i16,
	y: i16,
}

// Tuple struct
#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
struct Size(u16);

// Simple type definition
type Score = u32;
```

As you can see, Rustecs expects components to implement a bunch of basic traits.
This is required to provide some features that we don't need to concern
ourselves with at this point.

You might ask yourself why we're defining `Score` as `u32` and not just use
`u32` directly in the world definition above. While that should work (I haven't
actually tried it), it's not recommended. As we will see, the name of the
component's type is used to generate other names, for example the name of the
collection the components of a type are stored in.

You might also have different components that might be represented by the same
type, so you need the type alias to distinguish between them. Here's an example
of what this might look like:

``` Rust
// Two different component types that are represented by the same Rust type.
type Score = u32;
type Health = u32;
```


### Entities

Let's take another look at how entities are declared:

``` Rust
world! {
	entity_constructor asteroid(x: i16, y: i16, size: u16) -> (Position, Size) {
		(
			Position { x: x, y: y },
			Size(size),
		)
	}

	entity_constructor ship() -> (Position, Score) {
		(
			Position { x: 0, y: 0 },
			0,
		)
	}
}
```

Why are we declaring entity constructors here, not the entities themselves?
Well, contrary to components, entities in Rustecs aren't represented by a data
structure. An entity is just an id that refers to a bunch of components.

So why do we declare entity constructors instead of more generally declaring an
entity type? If you come from an object-oriented background, it might come
natural to you to think of an entity as having a "type", but that's not really
how an ECS works.

The "type" of an entity is solely defined by the components it has. When
implementing the logic for the ECS, you won't say "give me all ships" or "give
me all asteroids". You'd rather say "give me all entities that have a position
and a velocity" or something similar. It's even possible to add and remove
components, thereby changing the type of an entity at runtime (theoretically at
least, I haven't actually tried it in Rustecs).

The only thing that's specific to the "type" of entity is the way it is
constructed. Therefore we define entity constructors.

Given the world definition above, we can then create entities like this:

``` Rust
// world! { ... } generates a type called World. Here we create a new world.
let mut world = World::new();

// The ship contructor was defined without arguments.
world.create_ship();

// For an asteroid, we need to provide its position and size.
world.create_asteroid(8, -12, 50);

// The create functions return the id of the created entity. We can use that to
// destroy an entity later.
let ship_id = world.create_ship();
world.destroy_entity(ship_id);
```


### Systems

We' defined entity constructors and components, so we have all the tools we need
to populate our world with data. What we haven't done it is actually do
something with that data.

In an ECS, the logic of a game is implemented in systems. Systems are basically
just functions that operate on a set of entities. Those entities are defined by
the components they have.

As I'm writing this Rustecs doesn't have direct support for systems yet, but of
course you can still write systems that operate on your entities (otherwise
Rustecs would be pretty useless).

Let's look at this simple world:

``` Rust
world! {
	entity_constructor car() -> (Position, Velocity) {
		(
			Position { x: 0, y: 0 },
			Velocity { x: 0, y: 0 },
		)
	}
}

struct Position {
	x: i16,
	y: i16,
}
struct Velocity {
	x: i16,
	y: i16,
}
```

Here, we only have one entity constructor which constructs cars. Let's
initialize our world.

``` Rust
fn main() {
	// Create a world and add a bunch of cars to it.
	let world = World::new();
	world.create_car();
	world.create_car();
	world.create_car();

	// Do something with the cars.
	...
}
```

Of course, cars are no good if they don't move, so let's write a system for
that.

``` Rust
fn move_cars(positions: Components<Position>, velocities: Componenty<Velocity>) {
	for (entity_id, position) in positions.iter_mut() {
		if !velocity.contains_key(entity_id) {
			// There might be entities that have a position but no velocity.
			// Ignore those.
			continue;
		}

		// If we have both a position and a velocity, it's a car and supposed to
		// move!
		let velocity = velocities[entity_id];
		position.x += velocity.x;
		position.y += velocity.y;
	}
}
```

The system iterates over all entities with a `Position` component, checks if the
entity also has a `Velocity` component and integrates the position, if it has.

Currently, `Components<T>` is simply defined as `HashMap<EntityId, T>`.

So how do we call that system? Let's complete our main function from above.

``` Rust
fn main() {
	// Create a world and add a bunch of cars to it.
	let world = World::new();
	world.create_car();
	world.create_car();
	world.create_car();

	// Do something with the cars.
	loop {
		// For each component type we defined, the world has a collection. We
		// just pass those to the systems.
		move_cars(world.positions, world.velocities);

		// In a real game, we'd do other stuff in this loop, like gathering
		// player input and rendering the cars.
	}
}
```

As you can see, the facilities for using systems is pretty basic. I already have
some plans for adding proper system support to Rustecs, but I need some time to
design and implement that.


### That's It!

There are some additional feaures I haven't talked about here, like importing
and exporting entities, but as far as basic use cases go, that's pretty much it.

If you have any questions, feel free to [contact me](mailto:mail@hannobraun.de).


## License

Copyright (c) 2014, Hanno Braun

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
