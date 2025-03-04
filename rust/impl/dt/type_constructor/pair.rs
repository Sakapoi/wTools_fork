/// Internal namespace.
pub( crate ) mod private
{
  use crate::exposed::*;

  ///
  /// Pair type constructor.
  ///
  /// Should not be used directly. Instead use macro [crate::types!].
  ///

  #[ macro_export ]
  macro_rules! _pair
  {

    // pair Pair : < T1, T2 >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident :
      <
        $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )?,
        $ParamName2 : ident $( : $ParamTy2x1 : ident $( :: $ParamTy2xN : ident )* $( + $ParamTy2x2 : path )* )? $(,)?
      >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      #[ derive( type_constructor_derive_pair_meta::Pair ) ]
      $( #[ $Meta ] )*
      $Vis struct $Name
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
        $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?,
      >
      ( pub $ParamName1, pub $ParamName2 );

      // From Pair Into Element cant be implemented because of Rust restructions.

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : < T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident :
      <
        $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x3 : path )* )?,
        $ParamName2 : ident $( : $ParamTy2x1 : ident $( :: $ParamTy2xN : ident )* $( + $ParamTy2x3 : path )* )?,
        $ParamName3 : ident
      $( $Rest : tt )*
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Parametrized element should be pair and have either two or single elements\n",
          stringify!
          (
            $( #[ $Meta ] )*
            $Vis pair $Name :
            <
              $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?,
              $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )?,
              $ParamName3
            $( $Rest )*
          )
        )
      );
    };

    // pair Pair : Element1< T1, T2, ... >, Element2< T1, T2, ... >;

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident
      :
      $TypeSplit1x1 : ident $( :: $TypeSplit1xN : ident )*
      $( < $( $( $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )? ),+ )? > )?
      ,
      $TypeSplit2x1 : ident $( :: $TypeSplit2xN : ident )*
      $( < $( $ParamName2 : ident $( : $ParamTy2x1 : ident $( :: $ParamTy2xN : ident )* $( + $ParamTy2x2 : path )* )? ),* > )?
      $(,)?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      #[ derive( type_constructor_derive_pair_meta::Pair ) ]
      $( #[ $Meta ] )*
      $Vis struct $Name
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ , )? )?
        $( $( $ParamName2 $( : $ParamTy2x1 $( :: $ParamTy2xN )* $( + $ParamTy2x2 )* )? ),* )?
      >
      (
        pub $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        pub $TypeSplit2x1 $( :: $TypeSplit2xN )* < $( $( $ParamName2 ),* )? >,
      );
      
      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : < T1 >; // homopair

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident :
      <
        $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )? $(,)?
      >
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      #[ derive( type_constructor_derive_pair_meta::Pair ) ]
      $( #[ $Meta ] )*
      $Vis struct $Name
      <
        $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )?
      >
      ( pub $ParamName1, pub $ParamName1 );

      $crate::types!{ $( $( $Rest )* )? }
    };

    // pair Pair : Element1< T1, T2, ... >; // homopair

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis pair $Name : ident
      :
      $TypeSplit1x1 : ident $( :: $TypeSplit1xN : ident )*
      $( < $( $( $ParamName1 : ident $( : $ParamTy1x1 : ident $( :: $ParamTy1xN : ident )* $( + $ParamTy1x2 : path )* )? ),+ )? > )?
      $(,)?
      $( ; $( $Rest : tt )* )?
    )
    =>
    {
      #[ derive( type_constructor_derive_pair_meta::Pair ) ]
      $( #[ $Meta ] )*
      $Vis struct $Name
      <
        $( $( $( $ParamName1 $( : $ParamTy1x1 $( :: $ParamTy1xN )* $( + $ParamTy1x2 )* )? ),+ )? )?
      >
      (
        pub $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
        pub $TypeSplit1x1 $( :: $TypeSplit1xN )* < $( $( $( $ParamName1 ),+ )? )? >,
      );

      $crate::types!{ $( $( $Rest )* )? }
    };
  }

  //

  // trace_macros!( true );
  types!
  {

    ///
    /// Type constructor to wrap two types into a tuple.
    ///
    /// ### Sample
    /// ```
    /// let i32_and_f32_in_tuple = type_constructor::Pair::< i32, f32 >::from( ( 13, 13.0 ) );
    /// dbg!( i32_and_f32_in_tuple );
    /// // let vec_of_i32_in_tuple = type_constructor::Pair::< i32, f32 >::from( [ 13, 13.0 ] );
    /// ```
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    pub pair Pair : < T1, T2 >;

    ///
    /// Type constructor to wrap pair of the same type.
    ///
    /// ### Sample
    /// ```
    /// let two_i32_in_tuple = type_constructor::HomoPair::< i32 >::from( ( 13, 31 ) );
    /// dbg!( two_i32_in_tuple );
    /// let vec_of_i32_in_tuple = type_constructor::HomoPair::< i32 >::from( [ 13, 31 ] );
    /// ```
    ///

    #[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
    pub pair HomoPair : < T >;

  }
  // trace_macros!( false );

  pub use _pair;
  pub use type_constructor_derive_pair_meta;
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
    _pair,
  };
}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::private::
  {
    Pair,
    HomoPair,
    type_constructor_derive_pair_meta,
  };
}
