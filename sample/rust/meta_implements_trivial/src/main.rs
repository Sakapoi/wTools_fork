pub use implements::implements;

fn main()
{

  dbg!( implements!( 13_i32 => Copy ) );
  // < implements!( 13_i32 => Copy ) : true
  dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
  // < implements!( 13_i32 => Copy ) : false

}
