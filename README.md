# Semantics

The term _semantics_ here is used to describe a value / type that strongly _represents_ something else, tracking its
type in the type-system and carrying its representation as an integer value at runtime. Semantics allow to represent
something meaningful, like a _user_, a _field_, etc. by using integers underneath. The target audience is people writing
protocols between two systems that are using two different namespaces.

The main motivation for this crate is [luminance], which needs to be able to _represent_ attributes (vertex attribute
like a position, normal, color; fragment outputs, etc.) in two different namespaces:

- The Rust code, where those attributes are manipulated via regular Rust variables.
- GLSL code, which might use different names for those variables.

Semantics allow an indirection for both systems to “speak the same language” and “represent the same data.”

[luminance]: https://crates.io/crates/luminance
