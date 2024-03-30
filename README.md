# terrors

This crate is built around `OneOf`, which functions as
a form of anonymous enum that can be narrowed in ways
that may be familiar for users of TypeScript etc...

The cool thing about it is that it functions like a
type-level heterogenous set, where there's only one
actual value among the different possibilities.

Rather than having a giant ball of mud enum that
you never handle individual concerns from, the idea
of this is that you want to have a minimized set of
actual error types that may thread through the stack.

This is being considered as a possible replacement
for the nested Result style of error handling that
has been popular for fine grained error handling in
correctness critical projects. The nice thing about
this type-level set of possibilities is that any
specific type can be peeled off while narrowing
the rest of the types if the narrowing fails.
Both narrowing and broadening are based on type-level set checking.

So far this is just a one-day weekend project but
as I fight off the urge to pass out I imagine it
might be useful going forward, after a few likely
bugs in the type-level magic are sanded down and
other ergonomic improvements are made.

# Principles
* no macros
* error handling does not mean putting every error in a global ball of mud enum
  that never actually gets handled. it means giving users the tools to reason
  about what might actually be a concern at different levels of the stack,
  and avoid having local concerns propagate outside of the area of responsibility.
* don't impose too much ergonomic hardship on the caller that is responsible
  for handling errors and doing their own narrowing as they can take care of
  local concerns, or broaden the error as they punt it up the stack in
  cases where the caller may need to handle a variety of concerns. we'll
  see if this is actually all that ergonomic or not. it feels better than
  nested Results, but we'll see.

# Examples

```
use terrors::OneOf;

let one_of_3: OneOf<(String, u32, Vec<u8>)> = OneOf::new(5);

let narrowed_res: Result<u32, OneOf<(String, Vec<u8>)>> =
    one_of_3.narrow();

assert_eq!(5, narrowed_res.unwrap());
```

OneOf can also be broadened to a superset, checked at compile-time.

```
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
        match err.narrow::<Timeout, (AllocationFailure,), _>() {
            Ok(_timeout) => {},
            Err(one_of_others) => return Err(one_of_others.broaden()),
        }
    }

    Err(OneOf::new(RetriesExhausted))
}
```
