
/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use std::collections::HashMap;
  use wca::{ Type, Routine };

  ///
  /// Form CA commands grammar.
  ///

  pub fn grammar_form() -> Vec< wca::Command >
  {
     let publish_command = wca::Command::former()
    .hint( "Publish package on `crates.io`." )
    .long_hint( "Publish package on `crates.io`." )
    .phrase( "publish" )
    .subject( "A path to package. Should be a directory with file `Cargo.toml`.", Type::List( Type::String.into(), ',' ), true )
    .property( "dry", "Run command dry. Default is false.", Type::String, true )
    .property( "verbosity", "Setup level of verbosity.", Type::String, true )
    .property_alias( "verbosity", "v" )
    .form();

    let list_command = wca::Command::former()
    .hint( "List workspace packages." )
    .long_hint( "List workspace packages" )
    .phrase( "list" )
    .subject( "A path to directory with workspace config.", Type::Path, true )
    // .subject( "A path to directory with workspace config. Should be a glob.", Type::List( Type::Path.into() ), true )
    .property( "format", "Output format. It can be topological sorted list of crates or list + set of independent crates trees.\n               Variants: topsort, tree. Default is \"tree\".", Type::String, true )
    .property( "filter", "Filter output packages.\n               Variants: local, nothing. Default is \"nothing\".", Type::String, true )
    .property( "root_module", "Log dependency tree for selected module. Works in combination with option 'type:tree'", Type::String, true )
    .form();

    let create_table_command = wca::Command::former()
    .hint( "Generate table for main Readme.md file" )
    .long_hint( "Generate table for main Readme.md file" )
    .phrase( "readme.health.table.generate" )
    .form();

    vec!
    [
      publish_command,
      list_command,
      create_table_command,
    ]
  }

  ///
  /// Form CA commands executor.
  ///

  pub fn executor_form() -> HashMap< String, Routine >
  {
    use command::*;
    HashMap::from
    ([
      ( "publish".to_owned(), Routine::new( publish ) ),
      ( "list".to_owned(), Routine::new( list ) ),
      ( "readme.health.table.generate".to_owned(), Routine::new( table_generate ) ),
    ])
  }
}

crate::mod_interface!
{

  protected use grammar_form;
  protected use executor_form;

  /// List packages.
  layer list;
  /// Publish packages.
  layer publish;
  /// Generate tables
  layer table;
}
