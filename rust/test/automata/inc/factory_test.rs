use super::*;

//

tests_impls!
{
  #[ test ]
  fn node()
  {
    use TheModule::*;

    let mut factory = TheModule::canonical::NodeFactory::make();

    let n1 = factory.node_making( 1 );
    let n1b = factory.node( 1 );
    a_id!( n1, n1b.id() );
    dbg!( &n1 );

    let node1a = factory.node( 1 );
    let node1b = factory.node( 1 );
    a_id!( node1a, node1b );

    let node1a = factory.node( &1 );
    let node1b = factory.node( &&1 );
    a_id!( node1a, node1b );

  }

  //

  #[ test ]
  fn basic()
  {
    use TheModule::*;

    let mut factory = TheModule::canonical::NodeFactory::make();

    let a = factory.node_making( 1 );
    let b = factory.node_making( 2 );

    factory.node_add_edge_to_node( a, b );
    factory.node_extend_out_nodes( b, [ a, b ].into_iter() );

    dbg!( factory.node( a ) );
    dbg!( factory.node( b ) );

    let exp = hset![ b ];
    let got : HashSet< _ > = factory.out_nodes( a ).collect();
    a_id!( got, exp );

    let exp = hset![ a, b ];
    let got : HashSet< _ > = factory.out_nodes( b ).collect();
    a_id!( got, exp );
  }

  //

  #[ test ]
  fn make_with_edge_list()
  {
    use TheModule::*;

    let mut factory = TheModule::canonical::NodeFactory::make();

    factory.make_with_edge_list
    ([
      1, 2,
      2, 1,
      2, 2,
    ]);

    dbg!( factory.node( 1 ) );
    dbg!( factory.node( 2 ) );

    let exp = hset![ 2 ];
    let got : HashSet< _ > = factory.out_nodes( 1 ).collect();
    a_id!( got, exp );

    let exp = hset![ 1, 2 ];
    let got : HashSet< _ > = factory.out_nodes( 2 ).collect();
    a_id!( got, exp );
  }

  //

  #[ test ]
  fn make_with_edge_list_string()
  {
    use TheModule::*;

    let mut factory = TheModule::canonical::NodeFactory::make();

//     factory.make_with_edge_list
//     ([
//       "A", "B",
//       "B", "A",
//       "B", "B",
//     ]);
//
//     dbg!( factory.node( "A" ) );
//     dbg!( factory.node( "B" ) );

//     let exp = hset![ 2 ];
//     let got : HashSet< _ > = factory.out_nodes( 1 ).collect();
//     a_id!( got, exp );
//
//     let exp = hset![ 1, 2 ];
//     let got : HashSet< _ > = factory.out_nodes( 2 ).collect();
//     a_id!( got, exp );
  }

  //

  #[ test ]
  fn graph_print()
  {
    use TheModule::*;

    let mut factory = TheModule::canonical::NodeFactory::make();

    factory.make_with_edge_list
    ([
      1, 2,
      2, 1,
      2, 2,
    ]);

    let exp = r#"NodeFactory
  node::1
   - 2
  node::2
   - 1
   - 2"#;
    let got = format!( "{:?}", factory );
    println!( "{}", got );
    a_id!( got, exp );

  }

}

//

tests_index!
{
  node,
  basic,
  make_with_edge_list,
  make_with_edge_list_string,
  graph_print,
}
