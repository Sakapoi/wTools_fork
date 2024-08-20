use std::fmt::Debug;
use core::ops::Not;

#[ allow( dead_code ) ]
struct BoundsMixed< T : ToString + Not< Output = T >, U >
where
  U : Debug + Not< Output = U >,
{
  a : T,
  b : U,
}

impl< T : ToString + Not< Output = T >, U > Not for BoundsMixed< T, U >
where
  U : Debug + Not< Output = U >,
{
  type Output = Self;

  fn not( self ) -> Self::Output
  {
    Self { a : !self.a, b : !self.b }
  }
}

include!( "./only_test/bounds_mixed.rs" );
