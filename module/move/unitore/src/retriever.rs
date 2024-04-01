//! Client that fetches feeds entries.

use hyper_tls::HttpsConnector;
use hyper_util::
{
  client::legacy::Client,
  rt::TokioExecutor,
};
use http_body_util::{ Empty, BodyExt };
use hyper::body::Bytes;
use feed_rs::parser as feed_parser;
use error_tools::{ Result, for_app::Context };

// qqq : purpose of trait if any?
/// Fetch feed from provided source link.
#[ async_trait::async_trait ]
pub trait FeedFetch
{
  /// Get feed from source specified by its link.
  async fn fetch( &self, source : url::Url ) -> Result< feed_rs::model::Feed >;
}

/// Feed client for fetching feed.
#[ derive( Debug ) ]
pub struct FeedClient;

#[ async_trait::async_trait ]
impl FeedFetch for FeedClient
{
  async fn fetch( &self, source : url::Url ) -> Result< feed_rs::model::Feed >
  {
    let https = HttpsConnector::new();
    let client = Client::builder( TokioExecutor::new() ).build::< _, Empty< Bytes > >( https );
    let link = source.to_string().parse().context( format!( "Failed to parse source link {}", source ) )?;
    let mut res = client
    .get( link )
    .await
    .context( format!( "Failed to fetch frames from source {}", source ) )?
    ;

    let mut feed = Vec::new();
    while let Some( next ) = res.frame().await
    {
      let frame = next?;
      if let Some( chunk ) = frame.data_ref()
      {
        feed.extend( chunk.to_vec() );
      }
    }

    let feed = feed_parser::parse( feed.as_slice() ).context( "Failed to parse retrieved feeds." )?;

    Ok( feed )
  }
}
