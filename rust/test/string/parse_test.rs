
use wtest_basic::*;

#[cfg( feature = "in_wtools" )]
use wtools::string as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use wstring_tools as TheModule;

use TheModule::string::parse as parse;
use std::collections::HashMap;

//

fn _op_type_from_into()
{
  let got = parse::OpType::from( 1 );
  let exp = parse::OpType::Primitive( 1 );
  assert_eq!( got, exp );

  let got = parse::OpType::from( vec![ 1, 2 ] );
  let exp = parse::OpType::Vector( vec![ 1, 2 ] );
  assert_eq!( got, exp );

  /* */

  // let op = parse::OpType::from( 1 ); /* qqq : does not work properly, find better way to convert types */
  // let got : i32 = op.into();
  // assert_eq!( got, 1 );

  let op = parse::OpType::from( vec![ 1, 2 ] );
  let got : Vec<isize> = op.into();
  assert_eq!( got, vec![ 1, 2 ] );
}

//

fn _basic()
{
  let src = "";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = " ";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = " ";
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "  \t ";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = "  \t ";
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );
}

//

fn _with_subject_and_map()
{
  let src = "subj";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = "subj";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.maps = vec![ HashMap::new() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj with space";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = "subj with space";
  exp.subject = "subj with space".to_string();
  exp.subjects = vec![ "subj with space".to_string() ];
  exp.maps = vec![ HashMap::new() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj v:1";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:1";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj v:1 r:some";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
  options.insert( String::from( "r" ), parse::OpType::Primitive( String::from( "some" ) ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:1 r:some";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  /* */

  let src = "subj1 ; subj2";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut exp = parse::Request::default();
  exp.original = "subj1 ; subj2";
  exp.subject = "subj1".to_string();
  exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
  exp.maps = vec![ HashMap::new(), HashMap::new() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj1 v:1 ; subj2";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
  let mut exp = parse::Request::default();
  exp.original = "subj1 v:1 ; subj2";
  exp.subject = "subj1".to_string();
  exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone(), HashMap::new() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj1 v:1 ; subj2 v:2";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut options1 = HashMap::new();
  options1.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
  let mut options2 = HashMap::new();
  options2.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "2" ) ) );
  let mut exp = parse::Request::default();
  exp.original = "subj1 v:1 ; subj2 v:2";
  exp.subject = "subj1".to_string();
  exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
  exp.map = options1.clone();
  exp.maps = vec![ options1.clone(), options2.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj1 v:1 ne:-2 ; subj2 v:2 r:some";
  let req = TheModule::string::request_parse()
  .src( src )
  .perform();
  let mut options1 = HashMap::new();
  options1.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "1" ) ) );
  options1.insert( String::from( "ne" ), parse::OpType::Primitive( String::from( "-2" ) ) );
  let mut options2 = HashMap::new();
  options2.insert( String::from( "v" ), parse::OpType::Primitive( String::from( "2" ) ) );
  options2.insert( String::from( "r" ), parse::OpType::Primitive( String::from( "some" ) ) );
  let mut exp = parse::Request::default();
  exp.original = "subj1 v:1 ne:-2 ; subj2 v:2 r:some";
  exp.subject = "subj1".to_string();
  exp.subjects = vec![ "subj1".to_string(), "subj2".to_string() ];
  exp.map = options1.clone();
  exp.maps = vec![ options1.clone(), options2.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );
}

//

fn _with_several_values()
{
  let src = "subj v:1 v:2";
  let req = TheModule::string::request_parse()
  .src( src )
  .several_values( false )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Primitive( "2".to_string() ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:1 v:2";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj v:1 v:2";
  let req = TheModule::string::request_parse()
  .src( src )
  .several_values( true )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "1".to_string(), "2".to_string() ] ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:1 v:2";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );
}

//

fn _with_parsing_arrays()
{
  let src = "subj v:[1,2]";
  let req = TheModule::string::request_parse()
  .src( src )
  .parsing_arrays( false )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Primitive( "[1,2]".to_string() ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:[1,2]";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj v:[1,2]";
  let req = TheModule::string::request_parse()
  .src( src )
  .parsing_arrays( true )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "1".to_string(), "2".to_string() ] ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:[1,2]";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  /* */

  let src = "subj v:[1,2] v:3";
  let req = TheModule::string::request_parse()
  .src( src )
  .parsing_arrays( true )
  .several_values( true )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string() ] ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:[1,2] v:3";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj v:3 v:[1,2]";
  let req = TheModule::string::request_parse()
  .src( src )
  .parsing_arrays( true )
  .several_values( true )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "3".to_string(), "1".to_string(), "2".to_string() ] ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:3 v:[1,2]";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );

  let src = "subj v:[1,2] v:[3,4]";
  let req = TheModule::string::request_parse()
  .src( src )
  .parsing_arrays( true )
  .several_values( true )
  .perform();
  let mut options = HashMap::new();
  options.insert( String::from( "v" ), parse::OpType::Vector( vec![ "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string() ] ) );
  let mut exp = parse::Request::default();
  exp.original = "subj v:[1,2] v:[3,4]";
  exp.subject = "subj".to_string();
  exp.subjects = vec![ "subj".to_string() ];
  exp.map = options.clone();
  exp.maps = vec![ options.clone() ];
  exp.key_val_delimeter = ":";
  exp.commands_delimeter = ";";
  assert_eq!( req, exp );
}

//

test_suite!
{
  op_type_from_into,
  basic,
  with_subject_and_map,
  with_several_values,
  with_parsing_arrays,
}
