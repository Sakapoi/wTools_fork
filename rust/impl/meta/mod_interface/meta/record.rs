/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use derive_tools::IsVariant;
  use proc_macro_tools::exposed::*;

  ///
  /// Custom keywords.
  ///

  pub mod kw
  {
    super::syn::custom_keyword!( layer );
  }

  ///
  /// Kind of element.
  ///

  #[ derive( IsVariant, Debug, PartialEq, Eq, Clone, Copy ) ]
  pub enum ElementType
  {
    MicroModule( syn::token::Mod ),
    Layer( kw::layer ),
    Use( syn::token::Use ),
  }

  //

  impl syn::parse::Parse for ElementType
  {

    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let lookahead = input.lookahead1();
      let element_type = match()
      {
        _case if lookahead.peek( syn::token::Mod ) =>
        {
          ElementType::MicroModule( input.parse()? )
        },
        _case if lookahead.peek( syn::token::Use ) =>
        {
          ElementType::Use( input.parse()? )
        },
        _case if lookahead.peek( kw::layer ) =>
        {
          ElementType::Layer( input.parse()? )
        },
        _default =>
        {
          return Err( lookahead.error() )
        },
      };
      Ok( element_type )
    }

  }

  //

  impl quote::ToTokens for ElementType
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use ElementType::*;
      match self
      {
        MicroModule( e ) => e.to_tokens( tokens ),
        Use( e ) => e.to_tokens( tokens ),
        Layer( e ) => e.to_tokens( tokens ),
      }
    }
  }

  ///
  /// Record.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Record
  {
    pub attrs : AttributesOuter,
    pub vis : Visibility,
    pub element_type : ElementType,
    pub elements : syn::punctuated::Punctuated< Pair< AttributesOuter, syn::Path >, syn::token::Comma >,
    pub use_elements : Option< crate::UseTree >,
    pub semi : Option< syn::token::Semi >,
  }

  //

  impl syn::parse::Parse for Record
  {

    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {

      let attrs = input.parse()?;
      let vis = input.parse()?;
      let element_type = input.parse()?;
      let mut elements;
      let mut use_elements = None;

      match element_type
      {
        ElementType::Use( _ ) =>
        {
          use_elements = Some( input.parse()? );
          elements = syn::punctuated::Punctuated::new();
        },
        _ =>
        {
          if input.peek( syn::token::Brace )
          {
            let input2;
            let _brace_token = syn::braced!( input2 in input );
            elements = syn::punctuated::Punctuated::parse_terminated( &input2 )?;
          }
          else
          {
            let ident = input.parse()?;
            elements = syn::punctuated::Punctuated::new();
            elements.push( Pair::new( make!(), ident ) );
          }
        },
      }

      let lookahead = input.lookahead1();
      if !lookahead.peek( Token![ ; ] )
      {
        return Err( lookahead.error() );
      }

      let semi = Some( input.parse()? );
      Ok( Record
      {
        attrs,
        vis,
        element_type,
        elements,
        use_elements,
        semi,
      })

    }

  }

  //

  impl quote::ToTokens for Record
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.attrs.to_tokens( tokens );
      self.vis.to_tokens( tokens );
      self.element_type.to_tokens( tokens );
      self.elements.to_tokens( tokens );
      self.semi.to_tokens( tokens );
    }
  }

  ///
  /// Many records.
  ///

  pub type Records = Many< Record >;

  impl AsMuchAsPossibleNoDelimiter for Record {}

  ///
  /// Thesis.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Thesis
  {
    pub head : AttributesInner,
    pub records : Records,
  }

  //

  impl Thesis
  {
    /// Validate each inner attribute of the thesis.
    pub fn inner_attributes_validate( &self ) -> Result< () >
    {
      self.head.iter().try_for_each( | attr |
      {
        // code_print!( attr.path );
        // code_print!( attr.tokens );

        let good = true
          && code_export_str!( attr.path ) == "debug"
          && code_export_str!( attr.tokens ).is_empty()
        ;

        if !good
        {
          return Err( syn_err!
          (
            attr,
            "Unknown inner attribute:\n{}",
            tree_diagnostics_str!( attr ),
          ));
        }

        Result::Ok( () )
      })?;
      Ok( () )
    }
    /// Does the thesis has debug inner attribute.
    pub fn has_debug( &self ) -> bool
    {
      self.head.iter().any( | attr |
      {
        code_export_str!( attr.path ) == "debug"
      })
    }
  }

  //

  impl syn::parse::Parse for Thesis
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let head = input.parse()?;
      // let head = Default::default();
      let records = input.parse()?;
      Ok( Thesis
      {
        head,
        records,
      })
    }
  }

  //

  impl quote::ToTokens for Thesis
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      self.head.to_tokens( tokens );
      self.records.to_tokens( tokens );
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
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
  pub use super::prelude::*;
  pub use super::private::
  {
    ElementType,
    Record,
    Records,
    Thesis,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}
