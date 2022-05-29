/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use core::fmt::Debug;
  use core::hash::Hash;

  ///
  /// Kind of a ode.
  ///

  pub trait NodeKindInterface
  where
    Self :
      'static +
      Copy +
      Debug +
      PartialEq +
      Hash  +
      Default +
    ,
  {
  }

  impl< T > NodeKindInterface for T
  where
    T :
      'static +
      Copy +
      Debug +
      PartialEq +
      Hash  +
      Default +
    ,
  {
  }

  ///
  /// No kind for nodes.
  ///

  #[ derive( Debug, PartialEq, Copy, Clone, Hash, Default ) ]
  pub struct NodeKindless();

  ///
  /// Node of a graph.
  ///

  pub trait NodeBasicInterface
  where
    Self :
      // Hash +
      HasId +
  {

    // /// Iterate output nodes of the node.
    // fn out_nodes( &self ) -> Box< dyn Iterator< Item = < Self as HasId >::Id > + '_ >;
    // // fn out_nodes< 'a >( &'a self ) -> Box< dyn Iterator< Item = < Self as HasId >::Id > + 'a >;

  }

//   ///
//   /// Node which is extendable
//   ///
//
//   pub trait NodeExtendableInterface
//   where
//     Self :
//       Sized +
//       NodeBasicInterface +
//       Extend< Self > +
//     ,
//   {
//   }
//
//   impl< T > NodeExtendableInterface for T
//   where
//     T :
//       NodeBasicInterface +
//       Extend< Self > +
//     ,
//   {
//   }

//   ///
//   /// Node which has a kind.
//   ///
//
//   pub trait NodeKindGetterInterface< Kind >
//   where
//     Kind : NodeKindInterface,
//     Self : NodeBasicInterface,
//   {
//     /// Get kind of the node.
//     fn kind() -> Kind;
//   }

  ///
  /// Node handle.
  ///

  pub trait NodeHandleInterface : NodeBasicInterface + HasId
  {
    /// Node itself.
    type Node : NodeBasicInterface + HasId< Id = Self::Id >;
  }

}

/// Protected namespace of the module.
pub mod protected
{
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
  // use super::private as i;
  pub use super::prelude::*;
  pub use super::private::NodeKindless;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // use super::private as i;
  pub use super::private::NodeKindInterface;
  pub use super::private::NodeBasicInterface;
  // pub use super::private::NodeKindGetterInterface;
  pub use super::private::NodeHandleInterface;
}
