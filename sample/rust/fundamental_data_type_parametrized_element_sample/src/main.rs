use fundamental_data_type::*;

fn main()
{

  types!
  {
    #[ derive( Debug ) ]
    single MySingle : std::sync::Arc< T : Copy >;
  }
  let x = MySingle( std::sync::Arc::new( 13 ) );
  dbg!( x );

}
