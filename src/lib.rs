//! This crate is built around `OneOf`, which functions as
//! a form of anonymous enum that can be narrowed in ways
//! that may be familiar for users of TypeScript etc...
//!
//! # Examples
//!
//! ```
//! use terrors::OneOf;
//!
//! let one_of_3: OneOf<(String, u32, Vec<u8>)> = OneOf::new(5);
//!
//! let narrowed_res: Result<u32, OneOf<(String, Vec<u8>)>> =
//!     one_of_3.narrow();
//!
//! assert_eq!(5, narrowed_res.unwrap());
//! ```
//!
//! OneOf can also be broadened to a superset, checked at compile-time.
//!
//! ```
//! use terrors::OneOf;
//!
//! struct Timeout;
//! struct AllocationFailure;
//! struct RetriesExhausted;
//!
//! fn allocate_box() -> Result<Box<u8>, OneOf<(AllocationFailure,)>> {
//!     Err(AllocationFailure.into())
//! }
//!
//! fn send() -> Result<(), OneOf<(Timeout,)>> {
//!     Err(Timeout.into())
//! }
//!
//! fn allocate_and_send() -> Result<(), OneOf<(AllocationFailure, Timeout)>> {
//!     let boxed_byte: Box<u8> = allocate_box().map_err(OneOf::broaden)?;
//!     send().map_err(OneOf::broaden)?;
//!
//!     Ok(())
//! }
//!
//! fn retry() -> Result<(), OneOf<(AllocationFailure, RetriesExhausted)>> {
//!     for _ in 0..3 {
//!         let res = allocate_and_send();
//!         if res.is_ok() {
//!             return Ok(());
//!         }
//!
//!         let err = res.unwrap_err();
//!
//!         // keep retrying if we have a Timeout,
//!         // but punt allocation issues to caller.
//!         match err.narrow::<Timeout, (AllocationFailure,), _>() {
//!             Ok(_timeout) => {},
//!             Err(one_of_others) => return Err(one_of_others.broaden()),
//!         }
//!     }
//!
//!     Err(OneOf::new(RetriesExhausted))
//! }
//!
//! ```

mod one_of;
mod type_set;

pub use one_of::OneOf;
