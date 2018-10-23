Rust Lenses
===========

This project is inspired in the Lens functional design pattern for
changing immutable fields in algebraic data types. This project is
inspired on Julien Truffaut's Monocle library for Scala and Scala.js
(which in turn is inspired in Haskell's Lens).

Keep in mind that Rust does not support Higher-Kinded Types at the
moment, so not all functionality is available yet.


Include in your project
=======================

```
[dependencies]
lenses = "0.1"
```

```
#[macro_use]
extern crate lenses;
```


Description
-------------

Lenses are composable higher-order structures to manipulate data
types. Lenses have a `view` and a `set` function. A Lens is defined as
a `pub trait Lens<S, A>`, which means that a Lens can read and write
`A` values from any value of `S`.

The `pub fn lens<'a, S, A>(getter: &'a Fn(&S) -> A, setter: &'a Fn(&S,
&A) -> S) -> Lens<S, A>` will generate a lens given a function
(or reusable closure) of a getter and a setter.

Lenses can also be composed by means of `pub fn compose<'a, S, A,
B>(lhs: &'a Lens<S, A>, rhs: &'a Lens<A, B>) -> Lens<S, B>`.

All lenses are also `Getter<S, A>`, which is a weaker variant which
has only the getter part.

There are macros available for generating lenses for struct fields and
for any nested struct field as `struct_lens!` and `gen_lens!`,
respectively.

The macros expect that your lens is either `Clone` or `Copy`. Since
this pattern makes most sense for immutable data structures, it is
expected that such data structure will, at least, be `Clone`.

The syntax for the gen_lens macro goes like this:

```
#[derive(Clone, Copy, Debug)]
struct Example2 {
nested: i64,
}

#[derive(Clone, Copy, Debug)]
struct Example {
field: i64,
nested: Example2,
}

fn usage() {
  let e = Example { field: 4 };
  let lens = gen_lens!(copy Example, field);
  let lens2 = gen_lens!(copy Example2, nested);
  let lens3 = gen_lens!(copy Example, field.nested);
  let lens4 = compose(lens, lens2);

  // Operations over lens3 and lens4 are semantically equivalent
```

License
=======

Unless otherwise stated, code for this project is released under the
Apache Software License Version 2.0.


Contributing
============

This project uses the GitHub Flow approach for contributing, meaning
that we would really appreciate it if you would send patches as Pull
Requests in GitHub. If for any reason you prefer to send patches by
email, they are also welcome and will end up being integrated here.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be licensed as above, without any
additional terms or conditions.
