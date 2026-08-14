mod error;

#[cfg(feature = "reqwest")]
pub mod reqwest;
#[cfg(feature = "ureq")]
pub mod ureq;

pub use gitguardian_api as api;

pub use error::Error;
