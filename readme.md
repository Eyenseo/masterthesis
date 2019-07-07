# Abstract

The goal of this thesis is to provide a generator for SKilL that targets Rust.
Since Rusts type system enforces strict rules to offer guaranteed thread safety
the generated binding has to accommodate these rules, which makes it difficult
to create a graph with cycles. A lot of the effort goes into the research and
creation of solutions for the missing inheritance between structs and the
resulting missing casting functionality in the language. In conjunction an
option has to be found that allows Rusts rules to be enforced at runtime that
works with the enhanced inheritance support. Peculiarly, the resulting
implementation is not able to benefit from Rusts rules in regard to safe
parallelism, as the runtime enforcement of these rules inhibits parallelism. To
validate that the implementation works as expected pre-existing functional
tests are used. Additionally performance tests are executed to see how Rust
bindings fare against a reference implementation in C++.
