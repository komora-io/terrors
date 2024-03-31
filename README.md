# terrors - the Rust error **handling** library

Handling errors means taking a set of possible error
types, removing the ones that are locally addressible,
and then if the set of errors is not within those local
concerns, propagating the remainder to a caller. The
caller should not receive the local errors of the callee.

```rust
use terrors::OneOf;

let one_of_3: OneOf<(String, u32, Vec<u8>)> = OneOf::new(5);

let narrowed_res: Result<u32, OneOf<(String, Vec<u8>)>> =
    one_of_3.narrow();

assert_eq!(5, narrowed_res.unwrap());
```

# Principles

* Error types should be precise.
  * `terrors::OneOf` solves this by making precise sets of possible errors:
    * low friction to specify
    * low friction to narrow by specific error handlers
    * low friction to broaden to pass up the stack
* Error handling should follow the single responsibility principle
    * if every error in a system is spread everywhere else, there
      is no clear responsibility for where it needs to be handled.
* No macros.
    * Users should not have to learn some new DSL for error handling that every macro entails.

# Examples

```rust
use terrors::OneOf;

let one_of_3: OneOf<(String, u32, Vec<u8>)> = OneOf::new(5);

let narrowed_res: Result<u32, OneOf<(String, Vec<u8>)>> =
    one_of_3.narrow();

assert_eq!(5, narrowed_res.unwrap());
```

OneOf can also be broadened to a superset, checked at compile-time.

```rust
use terrors::OneOf;

struct Timeout;
struct AllocationFailure;
struct RetriesExhausted;

fn allocate_box() -> Result<Box<u8>, OneOf<(AllocationFailure,)>> {
    Err(AllocationFailure.into())
}

fn send() -> Result<(), OneOf<(Timeout,)>> {
    Err(Timeout.into())
}

fn allocate_and_send() -> Result<(), OneOf<(AllocationFailure, Timeout)>> {
    let boxed_byte: Box<u8> = allocate_box().map_err(OneOf::broaden)?;
    send().map_err(OneOf::broaden)?;

    Ok(())
}

fn retry() -> Result<(), OneOf<(AllocationFailure, RetriesExhausted)>> {
    for _ in 0..3 {
        let res = allocate_and_send();
        if res.is_ok() {
            return Ok(());
        }

        let err = res.unwrap_err();

        // keep retrying if we have a Timeout,
        // but punt allocation issues to caller.
        match err.narrow::<Timeout, _>() {
            Ok(_timeout) => {},
            Err(one_of_others) => return Err(one_of_others.broaden()),
        }
    }

    Err(OneOf::new(RetriesExhausted))
}
```

### Motivation

The paper [Simple Testing Can Prevent Most Critical Failures: An Analysis of Production Failures in Distributed Data-intensive Systems](https://www.eecg.toronto.edu/~yuan/papers/failure_analysis_osdi14.pdf)
is goldmine of fascinating statistics that illuminate the
software patterns that tend to correspond to system failures.
This is one of my favorites:

```no_compile
almost all (92%) of the catastrophic system failures
are the result of incorrect handling of non-fatal errors
explicitly signaled in software.
```

Our systems are falling over because we aren't handling
our errors. We're doing fine when it comes to signalling
their existence, but we need to actually handle them.

When we write Rust, we tend to encounter a variety of different
error types. Sometimes we need to put multiple possible errors
into a container that is then returned from a function, where
the caller or a transitive caller is expected to handle the
specific problem that arose.

As we grow a codebase, more of these situations pop up.
While it's not so much effort to write custom enums in
one or two places that hold the precise set of possible
errors, most people resort to one of two strategies for
minimizing the effort that goes into propagating their
error types:
* A large top-level enum that holds variants for errors
  originating across the codebase, tending to grow
  larger and larger over time, undermining the ability
  to use exhaustive pattern matching to confidently
  ensure that local concerns are not bubbling up the stack.
* A boxed trait that is easy to convert errors into, but
 then hides information about what may actually be inside.
 You don't know where it's been or where it's going.

As the number of different source error types that these
error containers hold increases, the amount of information
that the container communicates to people who encounter it
decreases. It becomes increasingly unclear what the error
container actually holds. As the precision of the type
goes down, so does a human's ability to reason about
where the appropriate place is to handle any particular
concern within it.

We have to increase the precision in our error types.

People don't write a precise enum for every function that
may only return some subset of errors because we would
end up with a ton of small enum types that only get used in
one or two places. This is the pain that drives people
to using overly-broad error enums or overly-smooth
boxed dynamic error traits, reducing their ability to
handle their errors.

### Cool stuff

This crate is built around `OneOf`, which functions as
a form of anonymous enum that can be narrowed in ways
that may be familiar for users of TypeScript etc...
Our error containers need to get smaller as individual
errors are peeled off and handled, leaving the reduced
remainder of possible error types if the local concerns
are not present.

The cool thing about it is that it is built on top of a
type-level heterogenous set of possible error types,
where there's only one actual value among the different
possibilities.

Rather than having a giant ball of mud enum or
boxed trait object that is never clear what it actually
contains, causing you to never handle individual
concerns from, the idea of this is that you can
have a minimized set of actual error types that may
thread through the stack.

The nice thing about this type-level set of possibilities
is that any specific type can be peeled off while narrowing
the rest of the types if the narrowing fails. Both narrowing
and broadening are based on compile-time error type set checking.
