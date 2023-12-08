#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/test_tools/latest/test_tools/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//!
//! Tools for writing and running tests.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// doc_file_test!( "rust/test/test/asset/Test.md" );

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  // zzz : exclude later
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::paste;
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::trybuild;
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::anyhow;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::rustversion;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::error_tools;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::meta_tools;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::mem_tools;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::typing_tools;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::num_traits;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::diagnostics_tools;

}

// #[ cfg( feature = "enabled" ) ]
// use ::meta_tools::mod_interface;

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
::meta_tools::mod_interface!
{
  // #![ debug ]

  layer test;

  use super::exposed::meta;
  use super::exposed::mem;
  use super::exposed::typing;
  use super::exposed::dt;
  use super::exposed::diagnostics;

  // protected use super::dependency;
  protected use super::dependency::*;

  prelude use ::rustversion::{ nightly, stable };

  prelude use ::meta_tools as meta;
  prelude use ::mem_tools as mem;
  prelude use ::typing_tools as typing;
  prelude use ::data_type as dt;
  prelude use ::diagnostics_tools as diagnostics;

  prelude use ::meta_tools::
  {
    impls,
    index,
    tests_impls,
    tests_impls_optional,
    tests_index,
  };
  prelude use ::typing_tools::{ implements };

}

// xxx : use module namespaces
#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
pub use test::{ compiletime, helper, smoke_test };
