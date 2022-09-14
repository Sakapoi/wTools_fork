/// Private namespace.
pub( crate ) mod private
{
  use std::fmt::{ Debug, Formatter };
  use crate::common::prelude::*;
  use crate::yuv;
  use wmath::X2;
  use ::ac_ffmpeg::
  {
    packet::PacketMut,
    codec::{ CodecParameters, VideoCodecParameters },
    format::
    {
      io::IO,
      muxer::{ Muxer, OutputFormat },
    },
    time::{ TimeBase, Timestamp },
    Error,
  };
  use openh264::encoder::{ Encoder, EncoderConfig };

  //

  /// Encoder for the buffer.
  // #[ derive( Former ) ]
  pub struct Mp4
  {
    /// Frame width and height.
    dims : X2< usize >,
    /// Frame rate.
    frame_rate : usize,
    #[ cfg( feature = "mp4_ratio_conversion" ) ]
    /// Frame rate multiplier.
    #[ cfg( feature = "mp4_ratio_conversion" ) ]
    frame_rate_ratio : usize,
    /// Frame index.
    frame_idx : i64,
    /// Time base of video.
    time_base : TimeBase,
    /// Color encoding.
    color_type : ColorType,
    /// Config for color format encoder.
    config : EncoderConfig,
    /// Muxer for the mp4.
    muxer : Muxer< std::fs::File >,
    /// Output filename.
    output_filename : std::path::PathBuf,
  }

  impl Debug for Mp4
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "Mp4" )
      .field( "width", &self.dims.0 )
      .field( "height", &self.dims.1 )
      .field( "frame_rate", &self.frame_rate )
      .field( "color_type", &self.color_type )
      .field( "output_filename", &self.output_filename )
      .finish()
     }
  }

  impl EncodeData for Mp4
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : &[ u8 ] ) -> Result< (), Box<dyn std::error::Error > >
    {
      let data = match self.color_type
      {
        ColorType::Rgb =>
        {
          data.to_vec()
        },
        ColorType::Rgba =>
        {
          /* skip alpha channel */
          data.iter().enumerate()
          .filter_map( | ( i, v ) | if ( i + 1 ) % 4 == 0 { None } else { Some( *v ) } )
          .collect::<Vec<u8>>()
        },
        // TODO: Avoid double conversion.
        ColorType::Yuv444 =>
        {
          yuv::yuv444_to_rgb( data )
        },
        ColorType::Yuv422 =>
        {
          yuv::yuv422_to_rgb( data )
        },
        ColorType::Yuv420p =>
        {
          yuv::yuv420p_to_rgb( data, self.dims.0, self.dims.1 )
        },
        ColorType::Yvu420p =>
        {
          yuv::yvu420p_to_rgb( data, self.dims.0, self.dims.1 )
        },
        ColorType::Yuv422p =>
        {
          yuv::yuv422p_to_rgb( data, self.dims.0, self.dims.1 )
        },
      };

      let frame_timestamp = Timestamp::new( self.frame_idx, self.time_base );
      self.frame_idx += 1;

      let mut yuv = openh264::formats::RBGYUVConverter::new( self.dims.0, self.dims.1 );
      yuv.convert( data.as_slice() );

      /* the initialization of new instance is required for correct conversion */
      let mut encoder = Encoder::with_config( self.config.clone() ).unwrap();
      let bitstream = encoder.encode( &yuv )?;
      let buf = bitstream.to_vec();

      #[ cfg( feature = "mp4_ratio_conversion" ) ]
      {
        let mut frame_timestamp = frame_timestamp;
        for _i in 0..self.frame_rate_ratio
        {
          let packet = PacketMut::from( &buf )
          .with_pts( frame_timestamp )
          .with_dts( frame_timestamp )
          .freeze();

          frame_timestamp = Timestamp::new( self.frame_idx, self.time_base );
          self.frame_idx += 1;
          self.muxer.push( packet )?;
        }
      }
      #[ cfg( not( feature = "mp4_ratio_conversion" ) ) ]
      {
        let packet = PacketMut::from( &buf )
        .with_pts( frame_timestamp )
        .with_dts( frame_timestamp )
        .freeze();
        self.muxer.push( packet )?;
      }

      Ok( () )

    }
    /// Finish encoding.
    fn flush( &mut self ) -> Result< (), Box<dyn std::error::Error > >
    {
      self.muxer.flush()?;
      Ok( () )
    }
  }

  impl Mp4
  {
    /// Create an instance.
    pub fn new
    (
      dims : X2< usize >,
      frame_rate : usize,
      _repeat : Option< usize >,
      color_type : &ColorType,
      filename : impl AsRef< str >
    ) -> Result< Self, Box< dyn std::error::Error > >
    {
      let path = filename.as_ref();
      let output_format = OutputFormat::guess_from_file_name( path )
      .ok_or_else( || Error::new( format!( "unable to guess output format for file: {}", path ) ) )?;

      let output = std::fs::File::create( path )
      .map_err( | err | Error::new( format!( "unable to create output file {}: {}", path, err ) ) )?;

      let io = IO::from_seekable_write_stream( output );

      let codec_parameters = CodecParameters::from
      (
        VideoCodecParameters::builder( "libx264" ).unwrap()
        .width( dims.0 )
        .height( dims.1 )
        .build()
      );

      let mut muxer_builder = Muxer::builder();
      muxer_builder.add_stream( &codec_parameters )?;
      let muxer = muxer_builder.build( io, output_format )?;

      #[ cfg( not( feature = "mp4_ratio_conversion" ) ) ]
      let base_frame_rate = frame_rate as u32;

      #[ cfg( feature = "mp4_ratio_conversion" ) ]
      let base_frame_rate = if frame_rate < 30
      {
        30
      }
      else
      {
        frame_rate as u32
      };
      let time_base = TimeBase::new( 1, base_frame_rate );

      let config = EncoderConfig::new( dims.0 as _, dims.1 as _ );

      let instance = Self
      {
        dims,
        frame_rate,
        #[ cfg( feature = "mp4_ratio_conversion" ) ]
        frame_rate_ratio : ( 30 / frame_rate ) as _,
        frame_idx : 0,
        time_base,
        color_type : color_type.clone(),
        config,
        muxer,
        output_filename : std::path::PathBuf::from( filename.as_ref() ),
      };
      Ok( instance )
    }
  }
}

//

wtools::meta::mod_interface!
{
  prelude use Mp4;
}
