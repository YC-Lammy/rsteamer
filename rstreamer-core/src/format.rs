
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Dimentions{
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat{
    Luma8,
    LumaA8,
    Luma16,
    LumaA16,
    Rgb8,
    Rgba8,
    Rgb16,
    Rgba16,
    CMYK8,
    CMYK16,
    Vendor(&'static str)
}

impl ImageFormat{
    pub fn has_alpha(&self) -> bool{
        match self{
            Self::LumaA16 |
            Self::LumaA8 |
            Self::Rgba16 |
            Self::Rgba8 => true,
            _ => false
        }
    }
}

pub struct TimeStamp(usize);

pub struct VideoFrameFormat{
    pub image_format: ImageFormat,
    pub resolution: Dimentions,
    /// NaN if static
    pub fps: f64,
}

pub struct AudioFrameFormat{
    pub channels: u32,
    pub sample_rate: f64,
}

pub enum SubtitleFormat{
    Text,
    Image
}

pub struct SubtitleFrameFormat{
    pub subtitle_format: SubtitleFormat,
    pub language: ()
}

pub struct VideoPacketFormat{
    pub encoding: VideoEncoding
}

pub struct AudioPacketFormat{
    pub encoding: AudioEncoding,
    pub channels: f64,
    pub sample_rate: f64,
}

pub struct SubtitlePacketFormat{
    pub encoding: SubtitleEncoding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoEncoding{
    RAW(ImageFormat),
    JPEG,
    PNG,
    WEBP,
    AV1,
    AVS2,
    H264,
    H265,
    VP8,
    VP9,
    Theora,
    Vendor(&'static str)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioEncoding{
    RAW,
    AAC,
    AC3,
    FLAC,
    OPUS,
    OPENCORE,
    VISUALON,
    VORBIS,
    WAVPACK,
    Vendor(&'static str)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubtitleEncoding{
    UTF8,
    UTF16,
    ARIB,
    DVDSUB,
    DVB,
    Vendor(&'static str)
}