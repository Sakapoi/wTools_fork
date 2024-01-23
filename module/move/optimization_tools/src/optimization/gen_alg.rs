//! Implementation of genetic algorithm for sudoku solving.
//! 
//! Initial population is generated by randomly filling every block in sudoku board with unique numbers.
//! 
//! Fitness is calculated as number of errors in board.
//!
//! New population is generated partially from fittest individuals( percent is determined by elite_selection_rate ),
//! partially from randomly chosen individuals( percent is determined by random_selection_rate ) and rest is generated
//! as offspring of crossover of random pair of individuals, selected by tournament method.
//! 
//! Tournament is performed by randomly selecting a group of individuals from the population( the number of individuals selected is equal to the tournament_size value).
//! Likelihood of win of the fittest participant is determined by tournament_selection_pressure.
//! 
//! Crossover is performed by combining blocks from parents' boards, split in several randomly chosen crossover points.
//! 
//! New population is modified by appling mutation to some individuals in the population. Individual's likelihood of being mutated id determined by mutation_rate value.
//! 
//! Termination: process is stopped if sudoku solution is found or if max_generation_number value is exseeded.
//! 

use std::{ collections::HashSet, fmt::Debug };

use deterministic_rand::Hrng;
use iter_tools::Itertools;
use rand::{ seq::SliceRandom, Rng };

use crate::{ sudoku::*, optimization::* };

/// Functionality of crossover genetic operator.
pub trait CrossoverOperator : Debug
{
  /// Produce new Individual using genetic matherial of two selected Individuals.
  fn crossover( &self, hrng : Hrng, parent1 : &SudokuPerson, parent2 : &SudokuPerson ) -> SudokuPerson;
}

/// Crossover is performed by combining blocks from parents' boards, split in several randomly chosen crossover points.
#[ derive( Debug ) ]
pub struct MultiplePointsBlockCrossover {}

impl CrossoverOperator for MultiplePointsBlockCrossover
{
  fn crossover( &self, hrng : Hrng, parent1 : &SudokuPerson, parent2 : &SudokuPerson ) -> SudokuPerson 
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let possible_values = [ 1, 2, 3, 4, 5, 6, 7, 8 ];
    let first_parent_blocks_number = possible_values.choose( &mut *rng ).unwrap();
    let mut first_parent_blocks : HashSet< BlockIndex > = HashSet::new();

    while first_parent_blocks.len() != *first_parent_blocks_number
    {
      first_parent_blocks.insert( rng.gen() );
    }

    let mut child_storage: Vec< CellVal > = vec![ 0.into(); 81 ];

    for i in parent1.board.blocks()
    {
      if first_parent_blocks.contains( &i )
      {
        let parent_block = parent1.board.block( i ).collect_vec();
        let cells = parent1.board.block_cells( i );
        for ( index, cell_index ) in cells.enumerate()
        {
          child_storage[ usize::from( cell_index ) ] = parent_block[ index ];
        }
      }
      else 
      {
        let parent_block = parent2.board.block( i ).collect_vec();
        let cells = parent2.board.block_cells( i );
        for ( index, cell_index ) in cells.enumerate()
        {
          child_storage[ usize::from( cell_index ) ] = parent_block[ index ];
        }
      }
    }

    let child = SudokuPerson::with_board( Board::new( child_storage ) );
    child
  }
}

/// Crossover performed by selecting blocks with best rows or columns from two Individuals.
#[ derive( Debug ) ]
pub struct BestRowsColumnsCrossover {}

impl CrossoverOperator for BestRowsColumnsCrossover
{
  fn crossover( &self, _hrng : Hrng, parent1 : &SudokuPerson, parent2 : &SudokuPerson ) -> SudokuPerson 
  {
    let mut rows_costs = vec![ Vec::new(); 2 ];
    let mut columns_costs = vec![ Vec::new(); 2 ];
    for ( index, parent ) in [ parent1, parent2 ].iter().enumerate()
    {
      rows_costs[ index ] = parent.board
      .rows()
      .map( | row | row.collect::< HashSet< _ > >().len() )
      .collect_vec()
      .chunks( 3 )
      .map( | costs | 27 - costs.iter().fold( 0, | acc, cost | acc + cost ) )
      .collect_vec()
      ;

      columns_costs[ index ] = parent.board
      .cols()
      .map( | row | row.collect::< HashSet< _ > >().len() )
      .collect_vec()
      .chunks( 3 )
      .map( | costs | 27 - costs.iter().fold( 0, | acc, cost | acc + cost ) )
      .collect_vec()
      ;
    }

    let mut child1_storage = vec![ CellVal::from( 0 ); 81 ];
    for i in 0..3
    {
      if rows_costs[ 0 ][ i ] < rows_costs[ 1 ][ i ]
      {
        for j in 0..3
        {
          let parent_block = parent1.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent1.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child1_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
      else
      {
        for j in 0..3
        {
          let parent_block = parent2.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent2.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child1_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
    }

    let mut child2_storage = vec![ CellVal::from( 0 ); 81 ];
    for i in 0..3
    {
      for j in 0..3
      {
        if columns_costs[ 0 ][ j ] < columns_costs[ 1 ][ j ]
        {
          let parent_block = parent1.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent1.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child2_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
        else 
        {
          let parent_block = parent2.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent2.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child2_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
    }

    let min_board = [ Board::new( child1_storage ), Board::new( child2_storage ) ]
    .into_iter()
    .min_by( | b1, b2 | b1.total_error().cmp( &b2.total_error() ) )
    .unwrap()
    ;

    SudokuPerson::with_board( min_board )   
  }
}

/// Performs selection of Individuals for genetic crossover and production of new Individual for next generation.
pub trait SelectionOperator : Debug
{
  /// Select Individuals which will be used by GA crossover and mutation operators for production of new individual.
  fn select< 'a >( &self, hrng : Hrng, population : &'a Vec< SudokuPerson > ) -> &'a SudokuPerson;
}

/// Selection operator which randomly selects a group of individuals from the population( the number of individuals selected is equal to the size value) and choosing the most fit with probability defined by selection_pressure value.
#[ derive( Debug ) ]
pub struct TournamentSelection 
{
  /// Number of Individuals selected to compete in tournament.
  pub size : usize,
  /// Probabilistic measure of a individuals likelihood of being selected in the tournament.
  pub selection_pressure : f64,
}

impl SelectionOperator for TournamentSelection
{
  fn select< 'a >( &self, hrng : Hrng, population : &'a Vec< SudokuPerson > ) -> &'a SudokuPerson 
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let mut candidates = Vec::new();
    for _ in 0..self.size
    {
      candidates.push( population.choose( &mut *rng ).unwrap() );
    }
    candidates.sort_by( | c1, c2 | c1.fitness().cmp( &c2.fitness() ) );

    let rand : f64 = rng.gen();
    let mut selection_pressure = self.selection_pressure;
    let mut winner = *candidates.last().unwrap();
    for i in 0..self.size
    {
      if rand < selection_pressure
      {
        winner = candidates[ i ];
        break;
      }
      selection_pressure += selection_pressure * ( 1.0 - selection_pressure );
    }
    winner
  }
}

/// Functionality of Individual(potential solution) for optimization with SA and GA.
pub trait Individual< G : Generation >
{
  /// Objective function value that is used to measure how close Individual solution is to optimum.
  fn fitness( &self ) -> usize;
  /// Recalculate fitness value of individual.
  fn update_fitness( &mut self );
  /// Optimize current Individual using GA or SA method as specified by mode.
  fn evolve< 'a >( &self, hrng : Hrng, generation : &G, mode : &EvolutionMode ) -> Self;
  /// Check if current solution is optimal.
  fn is_optimal( &self ) -> bool;
}

impl Individual< SudokuGeneration > for SudokuPerson
{
  fn is_optimal( &self ) -> bool
  {
    if self.cost == 0.into()
    {
      true
    }
    else 
    {
      false
    }
  }

  fn fitness( &self ) -> usize 
  {
    self.cost.into()
  }

  fn update_fitness( &mut self )
  {
    self.cost = self.board.total_error().into();
  }

  fn evolve< 'a >( &self, hrng : Hrng, generation : &SudokuGeneration, mode : &EvolutionMode ) -> SudokuPerson
  {
    match mode
    {
      EvolutionMode::GA 
      { 
        elite_selection_rate, 
        random_selection_rate, 
        max_stale_iterations : _, 
        fitness_recalculation : _, 
        mutation_rate, 
        crossover_operator, 
        selection_operator ,
      } => 
      {
        if generation.population.iter().position( | p | p == self ).unwrap() <= ( generation.population.len() as f64 * elite_selection_rate ) as usize
        {
          return self.clone();
        }
    
        let rng_ref = hrng.rng_ref();
        let mut rng = rng_ref.lock().unwrap();
    
        let rand : f64 = rng.gen();
        if rand < *random_selection_rate
        {
          return self.clone();
        }
        drop( rng );
        let parent1 = selection_operator.select( hrng.clone(), &generation.population );
        let parent2 = selection_operator.select( hrng.clone(), &generation.population );
        let child = crossover_operator.crossover( hrng.clone(), parent1, parent2 );
    
        let rng_ref = hrng.rng_ref();
        let mut rng = rng_ref.lock().unwrap();
        let rand : f64 = rng.gen();
        drop( rng );
        if rand < *mutation_rate
        {
          child.mutate_random( &self.board, hrng.clone() )
        }
        else 
        {
          child
        }
      },
      EvolutionMode::SA 
      { 
        temperature_decrease_factor: _, 
        temperature_increase_factor, 
        mutations_per_generation_limit, 
        resets_limit 
      } => 
      {
        let mut temperature = generation.temperature.unwrap();
        let mut n_mutations : usize = 0;
        let mut n_resets = 0;
        let mut expected_number_of_mutations = 4;
        let mut new_person = self.clone();

        loop
        {
          if n_mutations > *mutations_per_generation_limit
          {
            n_resets += 1;
            expected_number_of_mutations = 4;
            if n_resets >= *resets_limit
            {
              return self.clone();
            }
            let temperature2 = ( temperature.unwrap() + temperature_increase_factor.unwrap() ).into();
            log::trace!( " 🔄 reset temperature {temperature} -> {temperature2}" );
            sleep();
            temperature = temperature2;
            n_mutations = 0;
          }
  
          let rng_ref = hrng.rng_ref();
          let mut rng = rng_ref.lock().unwrap();
  
          let candidates = rayon::iter::repeat( () )
          .take( expected_number_of_mutations )
          .enumerate()
          .map( | ( i, _ ) | hrng.child( i ) )
          .flat_map( | hrng | 
            {
              let mutagen = self.mutagen( &generation.initial_board, hrng.clone() );
                   
              let mutagen_cross_cost = self.board.cross_error_for_value
              (
                mutagen.cell1, 
                self.board.cell( mutagen.cell2 ),
                mutagen.cell2, 
                self.board.cell( mutagen.cell1 )
              );
            
              let mut original_cross_cost = 0;
              original_cross_cost += self.board.cross_error( mutagen.cell1 );
              original_cross_cost += self.board.cross_error( mutagen.cell2 );
          
              let rng_ref = hrng.rng_ref();
              let mut rng = rng_ref.lock().unwrap();
          
              let cost_difference = 0.5 + mutagen_cross_cost as f64 - original_cross_cost as f64;
              let threshold = ( - cost_difference / temperature.unwrap() ).exp();
          
              log::trace!
              (
                "cost : {}  | cost_difference : {cost_difference} | temperature : {temperature}",
                self.cost,
              );
              let rand : f64 = rng.gen();
              let vital = rand < threshold;  
              if vital
              {
                let emoji = if cost_difference > 0.0
                {
                  "🔼"
                }
                else if cost_difference < 0.0
                {
                  "✔️"
                }
                else
                {
                  "🔘"
                };
                log::trace!( " {emoji} vital | rand( {rand} ) < threshold( {threshold} )" );
                if cost_difference == 0.0
                {
                  // sleep();
                }
                Some( mutagen )
              }
              else
              {
                log::trace!( " ❌ non-vital | rand( {rand} ) > threshold( {threshold} )" );
                None
              }
                
            } )
            .collect::< Vec< _ > >()
            ;

          let candidate = candidates.choose( &mut *rng );

          if let Some( mutagen ) = candidate
          {
            new_person.mutate( &mutagen );
            break;
          }

          n_mutations += expected_number_of_mutations;
          if expected_number_of_mutations < 32
          {
            expected_number_of_mutations += 4;
          }
        };
        new_person
      }
    }
    
  }
}

/// Fuctionality of operator responsible for creation of initial solutions generation.
pub trait SeederOperator
{
  /// Type that represents generation of solutions in optimization process.
  type Generation : Generation;

  /// Create the initial generation for the optimization algorithm.
  fn initial_generation( &self, hrng : Hrng, size : usize ) -> Self::Generation;
}

/// Functionality of generation of solutions for optimization.
pub trait Generation
{
  /// Performs evolution of generation, either as SA mutation of every Individual or using GA genetic operators defined in GAConfig.
  fn evolve< 'a >( &mut self, hrng : Hrng, mode : &'a EvolutionMode ) -> Self;

  /// Calculate initial temperature for SA optimization.
  fn initial_temperature( &self, hrng : Hrng ) -> Temperature;

  /// Check if current generation contains optimal solution.
  fn is_good_enough( &self ) -> bool;
}

impl Generation for SudokuGeneration
{
  fn evolve< 'a >( &mut self, hrng : Hrng, mode : &'a  EvolutionMode ) -> Self 
  {
    let mut new_population = Vec::new();
    self.population.sort_by( | p1, p2 | p1.fitness().cmp( &p2.fitness() ) );

    if let EvolutionMode::SA { temperature_decrease_factor, .. } = mode
    {
      if self.temperature.is_none()
      {
        self.temperature = Some( self.initial_temperature( hrng.clone() ) );
      }
      else 
      {
        self.temperature = Some( Temperature::from( self.temperature.unwrap() * ( 1.0f64 - temperature_decrease_factor.unwrap() ) ) );
      }
    }
    else 
    {
      if self.temperature.is_some()
      {
        self.temperature = None;
      }
    }

    for i in 0..self.population.len()
    {
      new_population.push( self.population[ i ].evolve( hrng.clone(), & *self, &mode ) );
      if new_population.last().unwrap().is_optimal()
      {
        break;
      }
    }
    
    SudokuGeneration
    {
      population : new_population,
      ..self.clone()
    }
  }

  /// Calculate the initial temperature for the optimization process.
  fn initial_temperature( &self, hrng : Hrng ) -> Temperature
  {
    use statrs::statistics::Statistics;
    let state = SudokuPerson::new( &self.initial_board, hrng.clone() );
    const N : usize = 16;
    let mut costs : [ f64 ; N ] = [ 0.0 ; N ];
    for i in 0..N
    {
      let state2 = state.mutate_random( &self.initial_board, hrng.clone() );
      costs[ i ] = state2.cost.into();
    }
    costs[..].std_dev().into()
  }

  fn is_good_enough( &self ) -> bool 
  {
    for person in &self.population
    {
      if person.is_optimal()
      {
        return true;
      }
    }
    false
  }
}
