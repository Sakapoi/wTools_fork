use super::*;
use TheModule::canonical::ReadableNodeFactory as ReadableNodeFactory;
use TheModule::canonical::GenerativeNodeFactory as GenerativeNodeFactory;

include!( "./factory_impls.rs" );

//

tests_index!
{
  node,
  basic,
  make_default,
  make_with_edge_list,
  // make_with_edge_list_string,
  graph_print,
}
