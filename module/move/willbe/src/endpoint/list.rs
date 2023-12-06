/// Internal namespace.
mod private
{
  use crate::*;
  use std::fmt::Formatter;
  use petgraph::
  {
    algo::toposort,
    algo::has_path_connecting,
    Graph
  };
  use std::path::PathBuf;
  use std::str::FromStr;
  // use anyhow::Context;
  use package::
  {
    FilterMapOptions,
    packages_filter_map
  };
  use wtools::error::
  {
    for_app::{ Error, Context },
    err
  };
  use cargo_metadata::
  {
    Dependency,
    DependencyKind,
    Package
  };
  use petgraph::prelude::{ Dfs, EdgeRef };
  use petgraph::visit::Topo;
  use workspace::Workspace;

  /// Args for `list` endpoint.
  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFormat
  {
    /// Tree like format.
    #[ default ]
    Tree,
    /// Topologically sorted list.
    Topological,
  }

  impl FromStr for ListFormat
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let value = match s
      {
        "tree" => ListFormat::Tree,
        "toposort" => ListFormat::Topological,
        e => return Err( err!( "Unknown format '{}'. Available values: [tree, toposort]", e ))
      };

      Ok( value )
    }
  }

  /// Args for `list` endpoint.
  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFilter
  {
    /// With all packages.
    #[ default ]
    Nothing,
    /// With local only packages.
    Local,
  }

  impl FromStr for ListFilter
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let value = match s
      {
        "nothing" => ListFilter::Nothing,
        "local" => ListFilter::Local,
        e => return Err( err!( "Unknown filter '{}'. Available values: [nothing, local]", e ) )
      };

      Ok( value )
    }
  }

  /// Output of the `list` endpoint
  #[ derive( Debug, Default, Clone ) ]
  pub enum ListReport
  {
    /// With tree format.
    Tree
    {
      /// Dependencies graph.
      graph : Graph< String, String >,
      /// Packages indexes to display.
      names : Vec< petgraph::stable_graph::NodeIndex >,
    },
    /// With topologically sorted list.
    List( Vec< String > ),
    /// Nothing to show.
    #[ default ]
    Empty
  }

  /// Wrapper to redirect output from `ptree` graph to `fmt::Write`
  pub( crate ) struct Io2FmtWrite< 'a, W >
  {
    pub f : &'a mut W,
  }

  impl< W : std::fmt::Write > std::io::Write for Io2FmtWrite< '_, W >
  {
    fn write( &mut self, buf : &[ u8 ] ) -> std::io::Result< usize >
    {
      use std::io::ErrorKind;

      let size = buf.len();

      self.f.write_str
      (
        std::str::from_utf8( buf )
        .map_err( | _ | std::io::Error::new( ErrorKind::InvalidData, "Allow only valid UTF-8 string" ) )?
      )
      .map_err( | e | std::io::Error::new( ErrorKind::Other, e ) )?;

      Ok( size )
    }

    fn flush( &mut self ) -> std::io::Result< () >
    {
      Ok( () )
    }
  }

  impl std::fmt::Display for ListReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        ListReport::Tree { graph, names } => for n in names
        {
          ptree::graph::write_graph_with(&graph, *n, Io2FmtWrite { f }, &ptree::PrintConfig::from_env() ).unwrap();
        },
        ListReport::List ( list ) => for ( i ,e ) in list.iter().enumerate() { writeln!( f, "{i}) {e}" )? },
        _ => {},
      }

      Ok( () )
    }
  }

  ///
  /// List workspace packages.
  ///

  pub fn list( path_to_manifest : PathBuf, format : ListFormat, filter : ListFilter ) -> Result< ListReport, ( ListReport, Error ) >
  {
    let mut report = ListReport::default();

    let manifest = manifest::open( &path_to_manifest.join( "Cargo.toml" ) ).context( "List of packages by specified manifest path" ).map_err( | e | ( report.clone(), e.into() ) )?;
    let mut metadata = Workspace::with_manifest_path( &path_to_manifest );

    let root_crate = manifest
    .manifest_data
    .as_ref()
    .and_then( | m | m.get( "package" ) )
    .map( | m | m[ "name" ].to_string().trim().replace( '\"', "" ) )
    .unwrap_or_default();


    // let packages_map =  metadata.packages.iter().map( | p | ( p.name.clone(), p ) ).collect::< HashMap< _, _ > >();

    let dep_filter: Option< Box< dyn Fn( &Package, &Dependency ) -> bool > > = match filter
    {
      ListFilter::Nothing =>
      // qqq: Dev dependencies do loop in the graph, but it would be great if there was some way around that
      {
        Some
          (
            Box::new( | _p: &Package, d: &Dependency | d.kind != DependencyKind::Development )
          )
      }
      ListFilter::Local =>
      {
        Some
        (
          Box::new( | _p: &Package, d: &Dependency | d.path.is_some() && d.kind != DependencyKind::Development )
        )
      }
    };

    let packages_map =  packages_filter_map
    (
      &metadata.load().packages_get(),
      FilterMapOptions{ dependency_filter: dep_filter, ..Default::default() }
    );

    let graph = graph::construct( &packages_map );

    let sorted = toposort( &graph, None ).map_err( | e | { use std::ops::Index; ( report.clone(), err!( "Failed to process toposort for package: {:?}", graph.index( e.node_id() ) ) ) } )?;

    match format
    {
      ListFormat::Tree if root_crate.is_empty() =>
      {
        let mut names = vec![ sorted[ 0 ] ];
        for node in sorted.iter().skip( 1 )
        {
          if names.iter().all( | name | !has_path_connecting( &graph, *name, *node, None ) ) && !names.contains( node )
          {
            names.push( *node );
          }
        }
        report = ListReport::Tree { graph : graph.map( | _, &n | String::from( n ), | _, &e | String::from( e ) ), names };
      },
      ListFormat::Tree =>
      {
        let names = sorted
        .iter()
        .filter_map( | idx | if graph.node_weight( *idx ).unwrap() == &&root_crate { Some( *idx ) } else { None } )
        .collect::< Vec< _ > >();

        report = ListReport::Tree { graph : graph.map( | _, &n | String::from( n ), | _, &e | String::from( e ) ), names };
      }
      ListFormat::Topological if root_crate.is_empty() =>
      {
        let names = sorted
        .iter()
        .rev()
        .map( | dep_idx | graph.node_weight( *dep_idx ).unwrap().to_string() )
        .collect::< Vec< String > >();

        report = ListReport::List( names );
      },
      ListFormat::Topological =>
      {
        let node = graph.node_indices().find( | n | graph.node_weight( *n ).unwrap() == &&root_crate ).unwrap();
        let mut dfs = Dfs::new( &graph, node );
        let mut subgraph = Graph::new();
        let mut node_map = std::collections::HashMap::new();
        while let Some( n )= dfs.next( &graph )
        {
          node_map.insert( n, subgraph.add_node( graph[ n ] ) );
        }

        for e in graph.edge_references()
        {
          if let ( Some( &s ), Some( &t ) ) = ( node_map.get( &e.source() ), node_map.get( &e.target() ) )
          {
            subgraph.add_edge( s, t, () );
          }
        }

        let mut topo = Topo::new( &subgraph );
        let mut names = Vec::new();
        while let Some( n ) = topo.next( &subgraph )
        {
          names.push( subgraph[ n ].clone() );
        }
        names.reverse();

        report = ListReport::List( names );
      }
    }

    Ok( report )
  }
}

// qqq : for Bohdan : move to tests folder
mod tests
{
  #[ test ]
  fn io2fmt_write()
  {
    use super::private::Io2FmtWrite;

    // Arrange
    fn accepts_io_write< W : std::io::Write >( mut w : W ) -> std::io::Result< () >
    {
      w.write( b"Hello, world!" )?;

      Ok( () )
    }

    let mut string = String::new();

    // Act
    accepts_io_write( Io2FmtWrite { f : &mut string } ).unwrap();

    // Assert
    assert_eq!( "Hello, world!", &string );
  }
}

//

crate::mod_interface!
{
  /// Argument for `list` endpoint. Sets the output format.
  protected use ListFormat;
  /// Argument for `list` endpoint. Sets filter(local or all) packages should be in the output.
  protected use ListFilter;
  /// Contains output of the endpoint.
  protected use ListReport;
  /// List packages in workspace.
  orphan use list;
}
