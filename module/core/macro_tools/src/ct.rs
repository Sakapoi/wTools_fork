//!
//! Compile-time const expressions.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

/// Compile-time const expressions for strings.
pub mod str;

/// Compile-time const expressions.
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::const_format::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  // pub use super::protected as ct;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
