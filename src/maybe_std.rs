#[cfg(not(feature = "std"))]
pub extern crate alloc;

pub mod error {
    #[cfg(not(feature = "std"))]
    pub use core::error::Error;
    #[cfg(feature = "std")]
    pub use std::error::Error;

    #[cfg(all(not(feature = "std"), feature = "error_provide"))]
    pub use core::error::Request;
    #[cfg(all(feature = "std", feature = "error_provide"))]
    pub use core::error::Request;
}

#[cfg(not(feature = "std"))]
pub use alloc::boxed::Box;
#[cfg(feature = "std")]
pub use std::boxed::Box;
