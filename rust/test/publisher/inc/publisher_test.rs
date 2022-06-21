
use super::*;

//

tests_impls!
{
  fn basic_no_args()
  {
    let path = std::ffi::OsStr::new( "../../../target/debug/wpublisher" );
    let proc = std::process::Command::new( path ).output().unwrap();
    assert!( !proc.status.success() );
    let stdout = std::str::from_utf8( proc.stdout.as_slice() ).unwrap();
    assert!( stdout.contains( "Illformed command" ) );
    assert!( stdout.contains( ".list - List packages." ) );
  }

  //

  fn basic_with_args()
  {
    let path = std::ffi::OsStr::new( "../../../target/debug/wpublisher" );
    let proc = std::process::Command::new( path ).arg( ".list" ).output().unwrap();
    assert!( proc.status.success() );
    let stdout = std::str::from_utf8( proc.stdout.as_slice() ).unwrap();
    assert_eq!( stdout, "" );
    let stderr = std::str::from_utf8( proc.stderr.as_slice() ).unwrap();
    assert_eq!( stderr, "" );
  }
}

//

tests_index!
{
  basic_no_args,
  basic_with_args,
}
