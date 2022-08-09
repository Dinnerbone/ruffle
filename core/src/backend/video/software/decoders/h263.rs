use crate::backend::video::software::VideoDecoder;
use h263_rs::parser::H263Reader;
use h263_rs::{DecoderOption, H263State, PictureTypeCode};
use h263_rs_yuv::bt601::yuv420_to_rgba;
use ruffle_types::backend::video::{DecodedFrame, EncodedFrame, Error, FrameDependency};

/// H263 video decoder.
pub struct H263Decoder(H263State);

impl H263Decoder {
    pub fn new() -> Self {
        Self(H263State::new(DecoderOption::SORENSON_SPARK_BITSTREAM))
    }
}

impl VideoDecoder for H263Decoder {
    fn preload_frame(&mut self, encoded_frame: EncodedFrame<'_>) -> Result<FrameDependency, Error> {
        let mut reader = H263Reader::from_source(encoded_frame.data());
        let picture = self
            .0
            .parse_picture(&mut reader, None)?
            .ok_or("Picture in video stream is not a picture")?;

        match picture.picture_type {
            PictureTypeCode::IFrame => Ok(FrameDependency::None),
            PictureTypeCode::PFrame => Ok(FrameDependency::Past),
            PictureTypeCode::DisposablePFrame => Ok(FrameDependency::Past),
            _ => Err("Invalid picture type code!".into()),
        }
    }

    fn decode_frame(&mut self, encoded_frame: EncodedFrame<'_>) -> Result<DecodedFrame, Error> {
        let mut reader = H263Reader::from_source(encoded_frame.data());

        self.0.decode_next_picture(&mut reader)?;

        let picture = self
            .0
            .get_last_picture()
            .expect("Decoding a picture should let us grab that picture");

        let (width, height) = picture
            .format()
            .into_width_and_height()
            .ok_or("H.263 decoder error!")?;
        let chroma_width = picture.chroma_samples_per_row();
        let (y, b, r) = picture.as_yuv();
        let rgba = yuv420_to_rgba(y, b, r, width.into(), chroma_width);
        Ok(DecodedFrame {
            width,
            height,
            rgba,
        })
    }
}

impl Default for H263Decoder {
    fn default() -> Self {
        Self::new()
    }
}
