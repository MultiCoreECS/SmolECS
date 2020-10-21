# SmolECS - A simple, parallel ECS implementation in Rust

Frequently found in game engines, an Entity Component System (ECS) structures game data in such a way that improves cache coherence and decreases code complexity.

SmolECS is a simple, parallel ECS implementation made to test two different kinds of ECSs. The first is the Archetypal method and the second has no readily available name so we call it the Storage method.

The Storage method stores each component in its own separate storage, with some method of indexing via an entity label. This is usually some form of vector, but hashmaps can be used for sparse lists or bitsets for simple labels. Every storage keeps track of which entries are valid; in the case of a vector, if a later entity has a component and a former does not, there will still be an entry in the vector for the former. If a system requires an entity with multiple components, it simply needs to iterate over each component storage and return entries that are valid. This method provides a large degree of flexibility as different components can use different storage types based on their frequency in the scene. It also allows for the quick addition and removal of components by simply updating an entry in the respective storage. This method is used in game engines such as Amethyst.

The Archetypal method stores mixed groups of components in vectors based on archetypes. For instance: all entities that contain solely Transforms and Rigidbodys are stored in one vector of Transform, Rigidbody tuples. If a system requires entities that have a Rigidbody and Transform, it simply needs to iterate over said Archetype vector and any other Archetype vector that contains both Rigidbodys and Transforms. This method offers better cache coherency than the Storage method, as system data is usually less spread out over memory. This method is used in game engines such as UnityDOTS and Bevy. While the Archetypal method seems to have better cache coherency, the process of adding and removing components can lead to reading and writing large amounts of memory. This cost could outweigh cache performance benefits in certain scenes and simulations.

In order to evaluate the performance of these two different kinds of ECSs, we will create three different experiments.

1. BouncingBalls: Balls bouncing around the screen with physics, collision, and changing properties.
2. SmolNBody: Gravity on particles with position and mass.
3. SmolTransform: Transform hierarchy with position, rotation, and scale.

We will test each experiment on each ECS method and compare the results.

## Building

Since this is a crate and not a program with a `main` loop, it can't be run alone, but the few basic tests we have can be run. With the Rust package manager Cargo, the crate can be built using the command

```
cargo build --release
```

To run the tests,

```
cargo test --release
```

can be used in this folder and each subcrate's folder. There are 13 tests in all.