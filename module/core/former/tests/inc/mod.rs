use super::*;

#[ allow( unused_imports ) ]
use test_tools::meta::*;

#[ cfg( not( feature = "no_std" ) ) ]
#[ path = "./all/basic_runtime_common.rs" ]
mod basic_runtime_common;

#[ path = "./all/alias.rs" ]
mod alias;
#[ path = "./all/basic.rs" ]
mod basic;
#[ path = "./all/conflict.rs" ]
mod conflict;
#[ path = "./all/string_slice_runtime.rs" ]
mod string_slice_runtime;
#[ path = "./all/string_slice.rs" ]
mod string_slice;

#[ path = "./all/default_user_type.rs" ]
mod default_user_type;
#[ path = "./all/user_type_no_default.rs" ]
mod user_type_no_default;
#[ path = "./all/user_type_no_debug.rs" ]
mod user_type_no_debug;
#[ path = "./all/default_primitive.rs" ]
mod default_primitive;
#[ path = "./all/default_primitive.rs" ]
mod unsigned_primitive_types;
#[ path = "./all/unsigned_primitive_types.rs" ]
mod default_container;
#[ path = "./all/perform.rs" ]
mod perform;

//

only_for_terminal_module!
{

  // stable have different information about error
  // that's why these tests are active only for nightly
  #[ test_tools::nightly ]
  #[ test ]
  fn trybuild_tests()
  {

    // use test_tools::dependency::trybuild;
    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    // let t = trybuild::TestCases::new();
    let t = test_tools::compiletime::TestCases::new();

    t.compile_fail( "tests/inc/all/former_bad_attr.rs" );
    t.pass( "tests/inc/all/former_hashmap_without_parameter.rs" );
    t.pass( "tests/inc/all/former_vector_without_parameter.rs" );

  }

}
