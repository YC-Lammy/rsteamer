use std::num::NonZeroU32;

use fast_image_resize::PixelType;

use rstreamer_core::error::Error;
use rstreamer_core::format::{Dimentions, ImageFormat, VideoFrameFormat}; 
use rstreamer_core::pipline::VideoPipeline;
use rstreamer_core::source::{VideoSink, Element, VideoSource, VideoFrame};

fn image_format_to_pixel_type(format: ImageFormat) -> PixelType{
    match format{
        ImageFormat::Vendor(_) => PixelType::U8,
        ImageFormat::Luma8 => PixelType::U8,
        ImageFormat::LumaA8 => PixelType::U8x2,
        ImageFormat::Luma16 => PixelType::U16,
        ImageFormat::LumaA16 => PixelType::U16x2,
        ImageFormat::CMYK16 => PixelType::U16,
        ImageFormat::CMYK8 => PixelType::U8,
        ImageFormat::Rgb8 => PixelType::U8x3,
        ImageFormat::Rgba8 => PixelType::U8x4,
        ImageFormat::Rgb16 => PixelType::U16x3,
        ImageFormat::Rgba16 => PixelType::U16x4,
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ScaleSampler{
    /// fastest algorithm
    Nearest,
    #[default]
    Bilinear,
    CatmullRom,
    Lanczos
}

pub struct Scale{
    dst_width: u32,
    dst_height: u32,
    src_width: u32,
    src_height: u32,
    sampler: ScaleSampler,
    format: Option<ImageFormat>,
    fps: f64
}

impl Scale{
    pub fn new(width: u32, height:u32, sampler: ScaleSampler) -> Self{
        Self { 
            dst_width: width, 
            dst_height:height, 
            src_width: 0,
            src_height: 0,
            sampler, 
            format: None, 
            fps: f64::NAN 
        }
    }
    fn resize(&self, data: &mut [u8]) -> Result<Box<[u8]>, Error>{
        if self.dst_width == 0 || self.dst_height == 0{
            return Ok(Box::new([]))
        }

        let format = self.format.expect("component Scale called without initialising");
        
        if self.src_width == 0 || self.src_height == 0{
            
        }

        let re = fast_image_resize::Image::from_slice_u8(
            NonZeroU32::new(self.src_width).expect("component Scale called without initialising"), 
            NonZeroU32::new(self.src_height).expect("component Scale called without initialising"), 
            data, 
            image_format_to_pixel_type(format)
        );
        let mut img = match re{
            Ok(v) => v,
            Err(e) => return Err(Error::error(e))
        };

        let mut dst = fast_image_resize::Image::new(
            NonZeroU32::new(self.dst_width).unwrap(), 
            NonZeroU32::new(self.dst_height).unwrap(), 
            image_format_to_pixel_type(format)
        );

        if self.sampler != ScaleSampler::Nearest && format.has_alpha(){
            let muldiv = fast_image_resize::MulDiv::default();
            muldiv.multiply_alpha_inplace(&mut img.view_mut()).unwrap();
        }

        let sampler = match self.sampler{
            ScaleSampler::Nearest => fast_image_resize::ResizeAlg::Nearest,
            ScaleSampler::Bilinear => fast_image_resize::ResizeAlg::Convolution(fast_image_resize::FilterType::Bilinear),
            ScaleSampler::CatmullRom => fast_image_resize::ResizeAlg::Convolution(fast_image_resize::FilterType::CatmullRom),
            ScaleSampler::Lanczos => fast_image_resize::ResizeAlg::Convolution(fast_image_resize::FilterType::Lanczos3)
        };

        let mut resizer = fast_image_resize::Resizer::new(sampler);

        resizer.resize(&img.view(), &mut dst.view_mut()).unwrap();

        if self.sampler != ScaleSampler::Nearest && format.has_alpha(){
            let muldiv = fast_image_resize::MulDiv::default();
            muldiv.divide_alpha_inplace(&mut img.view_mut()).unwrap();
        }

        return Ok(img.into_vec().into())
    }
}

impl Element for Scale{
    fn as_video_sink(&mut self) -> Option<&mut dyn VideoSink> {
        return Some(self)
    }
    fn as_video_source(&mut self) -> Option<&mut dyn rstreamer_core::source::VideoSource> {
        return Some(self)
    }
}

impl VideoSink for Scale{
    fn add_source(&mut self, format: rstreamer_core::format::VideoFrameFormat) -> Result<(), rstreamer_core::error::Error> {
        if let ImageFormat::Vendor(_) = format.image_format{
            return Err(Error::UnsupportedFormat)
        }

        if let Some(_) = self.format{
            return Err(Error::DuplicatedSource)
        }

        self.format = Some(format.image_format);
        self.fps = format.fps;
        self.src_width = format.resolution.width;
        self.src_height = format.resolution.height;

        Ok(())
    }
    fn on_push(&mut self, pipline: &VideoPipeline, mut frame: VideoFrame) -> Result<(), Error> {
        let re = self.resize(frame.raw.as_mut())?;
        pipline.push_frame(VideoFrame { raw: re, time_stamp: frame.time_stamp });

        return Ok(())
    }
}

impl VideoSource for Scale{
    fn format(&self) -> VideoFrameFormat {
        return VideoFrameFormat{
            image_format: self.format.unwrap(),
            resolution: Dimentions { width: self.dst_width, height: self.dst_height },
            fps: self.fps
        }
    }

    fn on_pull(&mut self, pipline: &VideoPipeline) -> Result<Option<VideoFrame>, Error> {
        match pipline.pull_frame(){
            Some(mut frame) => {
                let re = self.resize(&mut frame.raw)?;
                return Ok(Some(VideoFrame { raw: re, time_stamp: frame.time_stamp }))
            }
            None => return Ok(None)
        };
    }
}