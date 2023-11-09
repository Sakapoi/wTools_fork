mod private
{
  use std::
  {
    fs,
    path::PathBuf,
    collections::HashMap,
  };
  use cargo_metadata::
  {
    DependencyKind,
    Metadata,
    MetadataCommand,
    Package,
  };
  use petgraph::
  {
    graph::Graph,
    algo::toposort as pg_toposort,
  };
  use crate::tools::
  {
    manifest,
    process,
    digest,
    http,
  };
  use crate::version::bump;
  use anyhow::{ Context, Error, anyhow };

  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    get_info : Option< process::CmdReport >,
    bump : Option< String >,
    commit : Option< process::CmdReport >,
    push : Option< process::CmdReport >,
    publish : Option< process::CmdReport >,
  }

  ///
  /// Publish single packages.
  ///

  pub fn publish( current_path : &PathBuf, path : &PathBuf, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();

    let mut manifest = manifest::get( path ).map_err( | e | ( report.clone(), e ) )?;
    if !manifest.package_is() || manifest.local_is()
    {
      return Ok( report );
    }
    let data = manifest.manifest_data.as_deref_mut().ok_or( anyhow!( "Failed to get manifest data" ) ).map_err( | e | ( report.clone(), e ) )?;

    let mut package_dir = manifest.manifest_path.clone();
    package_dir.pop();

    let output = process::start_sync( "cargo package", &package_dir ).context( "Take information about package" ).map_err( | e | ( report.clone(), e ) )?;
    if output.err.contains( "not yet committed")
    {
      return Err(( report, anyhow!( "Some changes wasn't committed. Please, commit or stash that changes and try again." ) ));
    }
    report.get_info = Some( output );

    let name = &data[ "package" ][ "name" ].clone();
    let name = name.as_str().ok_or( anyhow!( "Package has no name" ) ).map_err( | e | ( report.clone(), e ) )?;
    let version = &data[ "package" ][ "version" ].clone();
    let version = version.as_str().ok_or( anyhow!( "Package has no version" ) ).map_err( | e | ( report.clone(), e ) )?;
    let local_package_path = local_path_get( name, version, &manifest.manifest_path );

    let local_package = fs::read( local_package_path ).context( "Read local package" ).map_err( | e | ( report.clone(), e ) )?;
    let remote_package = http::retrieve_bytes( name, version ).unwrap_or_default();

    let digest_of_local = digest::hash( &local_package );
    let digest_of_remote = digest::hash( &remote_package );

    if digest_of_local != digest_of_remote
    {
      if dry
      {
        report.bump = Some( "Bump package version".into() );

        let buf = format!( "git commit -am {}-v{}", name, version );
        let output = process::CmdReport
        {
          command : buf,
          path : current_path.clone(),
          out : String::new(),
          err : String::new(),
        };
        report.commit = Some( output );

        let buf = "git push".to_string();
        let output = process::CmdReport
        {
          command : buf,
          path : current_path.clone(),
          out : String::new(),
          err : String::new(),
        };
        report.push = Some( output );

        let buf = "cargo publish".to_string();
        let output = process::CmdReport
        {
          command : buf,
          path : package_dir.clone(),
          out : String::new(),
          err : String::new(),
        };
        report.publish = Some( output );
      }
      else
      {
        data[ "package" ][ "version" ] = bump( version ).map_err( | e | ( report.clone(), e ) )?;
        let version = &data[ "package" ][ "version" ].clone();
        let version = version.as_str().ok_or( anyhow!( "Failed to take package version after bump" ) ).map_err( | e | ( report.clone(), e ) )?;
        manifest.store().map_err( | e | ( report.clone(), e ) )?;
        report.bump = Some( "Bump package version".into() );

        let buf = format!( "git commit -am {}-v{}", name, version );
        let output = process::start_sync( &buf, current_path ).context( "Commit changes while publishing" ).map_err( | e | ( report.clone(), e ) )?;
        report.commit = Some( output );

        let buf = "git push".to_string();
        let output = process::start_sync( &buf, current_path ).context( "Push while publishing" ).map_err( | e | ( report.clone(), e ) )?;
        report.push = Some( output );

        let buf = "cargo publish".to_string();
        let output = process::start_sync( &buf, &package_dir ).context( "Publish" ).map_err( | e | ( report.clone(), e ) )?;
        report.publish = Some( output );
      }
    }

    Ok( report )
  }

  //

  pub fn filter( metadata : &Metadata ) -> HashMap< String, &Package >
  {
    let mut packages_map = HashMap::new();

    let _packages = metadata.packages.iter().filter( | package |
    {
      if package.publish.is_none()
      {
        packages_map.insert( package.name.clone(), *package );

        return true;
      }

      false
    }).collect::< Vec< _ > >();

    packages_map
  }

  //

  pub fn local_path_get< 'a >( name : &'a str, version : &'a str, manifest_path : &'a PathBuf ) -> PathBuf
  {
    let buf = format!( "package/{0}-{1}.crate", name, version );

    let package_metadata = MetadataCommand::new()
    .manifest_path( manifest_path )
    .exec()
    .unwrap();

    let mut local_package_path = PathBuf::new();
    local_package_path.push( package_metadata.target_directory );
    local_package_path.push( buf );

    local_package_path
  }

  //

  pub fn graph_build< 'a >( packages : &'a HashMap< String, &Package > ) -> Graph< &'a str, &'a str >
  {
    let mut deps = Graph::< &str, &str >::new();
    let _update_graph = packages.iter().map( | ( _name, package ) |
    {
      let root_node = if let Some( node ) = deps.node_indices().find( | i | deps[ *i ] == package.name )
      {
        node
      }
      else
      {
        deps.add_node( &package.name )
      };

      for dep in &package.dependencies
      {
        if dep.path.is_some() && dep.kind != DependencyKind::Development
        {
          let dep_node = if let Some( node ) = deps.node_indices().find( | i | deps[ *i ] == dep.name )
          {
            node
          }
          else
          {
            deps.add_node( &dep.name )
          };

          deps.add_edge( root_node, dep_node, &package.name );
        }
      }
    }).collect::< Vec< _ > >();

    deps
  }

  //

  pub fn toposort( packages : &HashMap< String, &Package > ) -> Vec< String >
  {
    let deps = graph_build( packages );

    let sorted = pg_toposort( &deps, None ).expect( "Failed to process toposort for packages" );
    let names = sorted
    .iter()
    .rev()
    .map( | dep_idx | deps.node_weight( *dep_idx ).unwrap().to_string() )
    .collect::< Vec< String > >();

    names
  }
}

//

crate::mod_interface!
{
  protected( crate ) use PublishReport;
  protected( crate ) use publish;

  protected( crate ) use filter;
  protected( crate ) use local_path_get;

  protected( crate ) use graph_build;
  protected( crate ) use toposort;
}
