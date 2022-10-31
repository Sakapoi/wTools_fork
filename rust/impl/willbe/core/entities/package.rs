/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;
  use toml::Value;

  /// Package
  #[ derive( Debug, Clone ) ]
  pub struct Package
  {
    path : PathBuf,
  }

  impl TryFrom< PathBuf > for Package
  {
    type Error = ();

    fn try_from( path : PathBuf ) -> Result< Self, Self::Error >
    {
      let config_str = std::fs::read_to_string( path.join( "Cargo.toml" ) ).or( Err( () ) )?;
      let toml = config_str.parse::< Value >().or( Err( () ) )?;

      if toml.get( "package" ).is_some()
      {
        Ok( Self{ path } )
      }
      else
      {
        Err( () )
      }
    }
  }

  impl Package
  {
    /// Gets path of package
    pub fn path( &self ) -> &PathBuf
    {
      &self.path
    }
  }
}

//

wtools::meta::mod_interface!
{
  prelude use Package;
}
