/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use core::fmt::Debug;

  ///
  /// Interface of a type responsible for constructing nodes.
  ///

  pub trait NodeFactoryInterface
  where
    Self : Debug,
  {
    // /// Node of the graph.
    // type Node : NodeBasicInterface;
    /// It's not always possible to operate a node directly, for example it it has to be wrapped by cell ref. For that use NodeHandle.
    /// Otherwise NodeHandle is the same as Node.
    type NodeHandle : NodeBasicInterface;
  }

}

/// Protected namespace of the module.
pub mod protected
{
  // // use super::private as i;
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  // // use super::private as i;
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  // // use super::private as i;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // use super::private as i;
  pub use super::private::NodeFactoryInterface;
}
