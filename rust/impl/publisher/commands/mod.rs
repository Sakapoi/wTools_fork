
//!
//! Library of utility to work with commands.
//!

/// Protected namespace of the module.
pub mod protected
{
}

/// Orphan namespace of the module.
pub mod orphan
{
}

/// Exposed namespace of the module.
pub mod exposed
{
}

/// Prelude namespace of the module.
pub mod prelude
{
}


/// Publish module.
#[ cfg( feature = "use_std" ) ]
mod publish;

/// List packages.
#[ cfg( feature = "use_std" ) ]
mod list;

/// Init aggregator commands.
#[ cfg( feature = "use_std" ) ]
mod init;
#[ cfg( feature = "use_std" ) ]
pub use init::*;

