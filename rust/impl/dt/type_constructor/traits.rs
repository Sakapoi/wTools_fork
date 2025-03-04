/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Clone as tuple.
  ///

  pub trait CloneAsTuple< Tuple >
  {
    /// Clone as tuple.
    fn clone_as_tuple( &self ) -> Tuple;
  }

  ///
  /// Clone as array.
  ///

  pub trait CloneAsArray< T, const N : usize >
  {
    /// Clone as array.
    fn clone_as_array( &self ) -> [ T ; N ];
  }

  ///
  /// Reinterpret as tuple.
  ///

  pub trait AsTuple< Tuple >
  {
    /// Reinterpret as tuple.
    fn as_tuple( &self ) -> &Tuple;
  }

  ///
  /// Reinterpret as array.
  ///

  pub trait AsArray< T, const N : usize >
  {
    /// Reinterpret as array.
    fn as_array( &self ) -> &[ T ; N ];
  }

  ///
  /// Reinterpret as slice.
  ///

  pub trait AsSlice< T >
  {
    /// Reinterpret as slice.
    fn as_slice( &self ) -> &[ T ];
  }

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
}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::private::
  {
    CloneAsTuple,
    CloneAsArray,
    AsTuple,
    AsArray,
    AsSlice,
  };
}
