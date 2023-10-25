
pub( crate ) mod private
{
  use smart_default::SmartDefault;


  ///
  /// Options for isolate.
  ///

  #[ allow( dead_code ) ]
  #[ derive( Debug, SmartDefault ) ]
  // #[ perform( fn isolate( &self ) -> ( &'a str, Option<&'a str>, &'a str ) ) ]
  pub struct IsolateOptions<'a>
  {
    #[ default( "" ) ]
    src : &'a str,
    #[ default( " " ) ]
    delimeter : &'a str,
    #[ default( true ) ]
    quote : bool,
    #[ default( true ) ]
    left : bool,
    #[ default( 1 ) ]
    times : u8, /* rrr : Dmytro : former do not form u16, u32, u64, usize, replace after fix */
    #[ default( true ) ]
    none : bool,
  }

  ///
  /// Adapter for IsolateOptions.
  ///

  pub trait IsolateOptionsAdapter< 'a >
  {
    /// Do isolate.
    fn isolate( &self ) -> ( &'a str, Option<&'a str>, &'a str )
    where
      Self : Sized,
    {
      ( "", None, "" )
    }
  }

  impl< 'a > IsolateOptionsAdapter< 'a > for IsolateOptions< 'a >
  {
    fn isolate( &self ) -> ( &'a str, Option<&'a str>, &'a str )
    {
      let times = self.times + 1;
      let result;

      /* */

      let left_none_result = | src : &'a str | -> ( &'a str, Option<&'a str>, &'a str )
      {
        if self.none
        {
          ( "", None, src )
        }
        else
        {
          ( src, None, "" )
        }
      };

      /* */

      let right_none_result = | src : &'a str | -> ( &'a str, Option<&'a str>, &'a str )
      {
        if self.none
        {
          ( src, None, "" )
        }
        else
        {
          ( "", None, src )
        }
      };

      /* */

      let count_parts_len = | parts : &Vec<&str> | -> usize
      {
        let mut len = 0;
        for i in 0..self.times
        {
          let i = i as usize;
          if i > 0
          {
            len += self.delimeter.len();
          }
          len += parts[ i ].len();
        }
        len
      };

      if self.left
      {
        let parts : Vec<&str> = self.src.trim().splitn( times.into(), self.delimeter ).collect();
        if parts.len() == 1
        {
          result = left_none_result( parts[ 0 ] );
        }
        else
        {
          let len = count_parts_len( &parts );
          let max_len = len + self.delimeter.len();
          if max_len <= self.src.len()
          {
            result = ( &self.src[ 0..len ], Some( self.delimeter ), &self.src[ max_len.. ] );
          }
          else
          {
            result = left_none_result( self.src );
          }
        }
      }
      else
      {
        let parts : Vec<&str> = self.src.trim().rsplitn( times.into(), self.delimeter ).collect();
        if parts.len() == 1
        {
          result = right_none_result( parts[ 0 ] );
        }
        else
        {
          let len = count_parts_len( &parts );
          if len + self.delimeter.len() <= self.src.len()
          {
            result = ( parts[ parts.len() - 1 ], Some( self.delimeter ), &self.src[ self.src.len() - len.. ] );
          }
          else
          {
            result = right_none_result( self.src );
          }
        }
      }

      result
    }
  }

  ///
  /// Function to split a string with some delimeter.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///

  pub fn isolate<'a>() -> IsolateOptions<'a>
  {
    IsolateOptions::default()
  }

  ///
  /// Function to split a string with some delimeter. Routine splits string from left.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///

  pub fn isolate_left<'a>() -> (&'a str, bool, &'a str)
  {
    let mut isolate_options = IsolateOptions::default();
    isolate_options.left = true;
    (isolate_options.src, isolate_options.left, isolate_options.delimeter)
  }

  ///
  /// Function to split a string with some delimeter. Routine splits string from right.
  ///
  /// It produces former. To convert former into options and run algorithm of splitting call `perform()`.
  ///

  pub fn isolate_right<'a>() -> (&'a str, bool, &'a str)
  {
    let mut isolate_options = IsolateOptions::default();
    isolate_options.left = false;
    (isolate_options.src, isolate_options.left, isolate_options.delimeter)
  }
}

/// Owned namespace of the module.
pub mod protected
{
  use super::private as i;

  pub use i::IsolateOptions;
  pub use i::IsolateOptionsAdapter;
  pub use i::isolate;
  pub use i::isolate_left;
  pub use i::isolate_right;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::private as i;

  pub use i::IsolateOptionsAdapter;
  pub use i::isolate;
  pub use i::isolate_left;
  pub use i::isolate_right;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  use super::private as i;

  pub use i::IsolateOptionsAdapter;
}
