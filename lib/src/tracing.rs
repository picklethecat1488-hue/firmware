//! Conditional compilation tracing facade.
//! Re-exports `tracing-defmt` when target tracing is enabled, otherwise defines no-op mock versions.

#![allow(unused_imports)]

#[cfg(feature = "tracing")]
pub use tracing_defmt::{self, *};

#[cfg(not(feature = "tracing"))]
pub use defmt::{debug, error, info, trace, warn};
#[cfg(not(feature = "tracing"))]
pub use tracing_mock_macros::instrument;
