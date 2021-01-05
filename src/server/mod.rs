/// catchers for unavailable routes
pub(crate) mod catchers;

/// core functions
pub(crate) mod core;

/// initialise the rocket server
mod init;
pub use init::start;

// Server Errors
pub mod errors;

pub mod response;
