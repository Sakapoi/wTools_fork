/// Internal namespace.
mod private
{
  use crate::list::logic::*;

  use crate::tools::*;
  use std::env;
  use wtools::error::Result;
  use cargo_metadata::
  {
    MetadataCommand,
  };
  use petgraph::
  {
    algo::toposort,
    algo::has_path_connecting,
  };
  use std::path::PathBuf;

  ///
  /// List packages.
  ///

  pub fn list(patterns : Vec< String > ) -> Result< () >
  {
    let current_path = env::current_dir().unwrap();

    let paths = files::find( current_path, patterns.as_slice() );
    let paths = paths.iter().filter_map( | s | if s.ends_with( "Cargo.toml" ) { Some( s ) } else { None } );

    for path in paths
    {
      let manifest = manifest_get( path );
      if manifest.package_is()
      {
        let local_is = manifest.local_is();
        let remote = if local_is { "local" } else { "remote" };
        let data = manifest.manifest_data.as_ref().unwrap();
        println!( "{} - {:?}, {}", data[ "package" ][ "name" ].to_string().trim(), path.parent().unwrap(), remote );
      }
    }

    Ok( () )
  }

  ///
  /// List workspace packages.
  ///

  pub fn workspace_list( path_to_workspace : PathBuf, root_crate : &str, list_type : &str ) -> Result< () >
  {
    let mut manifest = manifest::Manifest::new();
    let manifest_path = manifest.manifest_path_from_str( &path_to_workspace ).unwrap();
    let package_metadata = MetadataCommand::new()
    .manifest_path( &manifest_path )
    .no_deps()
    .exec()
    .unwrap();

    let packages_map = packages_filter( &package_metadata );
    let graph = graph_build( &packages_map );
    let sorted = toposort( &graph, None ).unwrap();

    if list_type == "tree" {
      if root_crate.is_empty() {
        let mut names = vec![ sorted[ 0 ] ];
        for node in sorted.iter().skip( 1 )
        {
          if names.iter().all( | name | !has_path_connecting( &graph, *name, *node, None ) ) && !names.contains( node )
          {
            names.push( *node );
          }
        }
        names.iter().for_each( | n | ptree::graph::print_graph( &graph, *n ).unwrap() );
      }
      else
      {
        sorted
        .iter()
        .filter_map( | idx | if graph.node_weight( *idx ).unwrap() == &root_crate { Some( *idx ) } else { None } )
        .for_each( | e | ptree::graph::print_graph(&graph, e ).unwrap() );
      }
    }
    else
    {
      let names = sorted
      .iter()
      .rev()
      .map( | dep_idx | graph.node_weight( *dep_idx ).unwrap().to_string() )
      .collect::< Vec< String > >();

      names.iter().enumerate().for_each( | ( i, e ) | println!( "{i}) {e}" ) );
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  /// List packages.
  prelude use list;
  /// List packages in workspace.
  prelude use workspace_list;
}
