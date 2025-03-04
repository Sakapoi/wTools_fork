/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Type constructor of many.
  ///
  /// Should not be used directly. Instead use macro [crate::types!].
  /// Type constructor `many` is available if eiter feature `use_std` or feature `use_alloc` is enabled. Also feature `many` should be enabled.
  ///

  #[ macro_export ]
  macro_rules! _many
  {
    ( $( $Rest:tt )* )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Type constructor `many` is available if eiter feature `use_std` or feature `use_alloc` is enabled. Also feature `many` should be enabled.\n",
        )
      );
    }
  }

  pub use _many;
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  pub use super::private::
  {
    _many,
  };

}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
