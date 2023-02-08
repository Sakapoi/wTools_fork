pub( crate ) mod private
{
  use std::borrow::Cow;
  use nom::
  {
    bytes::complete::take_while,
    IResult,
  };

  /// Parser configuration
  #[ derive( Debug ) ]
  pub struct Parser
  {
    /// Symbol that will be interpreted as the beginning of a command
    /// 
    /// command_delimeter = '.'
    /// ".command" -> Command( "command" )
    pub command_delimeter : char,
    /// Symbol that will be interpreted as a separator for the name and value of the property
    /// 
    /// prop_delimeter = ':'
    /// "prop:value" -> ( "prop", "value" )
    pub prop_delimeter : char,
    /// String that will be interpreted as a separator for namespaces
    /// 
    /// namespace_delimeter = ".also"
    /// "<commands1> .also <commads2>" -> Namespace( < commands1 > ), Namespace( < commands2 > )
    pub namespace_delimeter : Cow< 'static, str >,
  }

  /// Parses first word from string. All characters before first space
  pub fn any_word( input : &str ) -> IResult< &str, &str >
  {
    take_while( | c : char | !c.is_whitespace() )( input )
  }
}

//

crate::mod_interface!
{
  prelude use Parser;
  protected use any_word;
}
