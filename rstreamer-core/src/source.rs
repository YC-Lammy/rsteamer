use core::time::Duration;

use crate::error::Error;
use crate::pipline::{SubtitlePipeline, AudioPipeline, VideoPipeline};
use crate::format::*;

pub struct VideoPacket{
    pub data: Box<[u8]>,
    pub time_stamp: TimeStamp
}

pub struct AudioPacket{
    pub data: Box<[u8]>,
    pub time_stamp: TimeStamp,
}

pub struct SubtitlePacket{
    pub data: Box<[u8]>,
    pub time_stamp: TimeStamp
}

pub struct VideoFrame{
    pub raw: Box<[u8]>,
    pub time_stamp: TimeStamp
}

pub struct AudioFrame{
    pub data: Box<[i16]>,
    pub time_stamp: TimeStamp
}

pub enum SubtitleFrame{
    Text{
        text: String,
        duration: Duration
    },
    Image
}

pub trait VideoDecoder: VideoSource{
    fn config(&mut self, format: VideoPacketFormat) -> Result<(), Error>;
}

pub trait AudioDecoder: AudioSource{
    fn config(&mut self, format: AudioPacketFormat) -> Result<(), Error>;
}

pub trait SubtitleDecoder: SubtitleSource{
    fn config(&mut self, format: SubtitlePacketFormat) -> Result<(), Error>;
}

pub trait VideoSource: Element{
    fn format(&self) -> VideoFrameFormat;
    fn on_pull(&mut self, pipline: &VideoPipeline) -> Result<Option<VideoFrame>, Error>;
}

pub trait VideoSink: Element{
    fn add_source(&mut self, format: VideoFrameFormat) -> Result<(), Error>;
    fn on_push(&mut self, pipline: &VideoPipeline, frame: VideoFrame) -> Result<(), Error>;
}

pub trait AudioSource: Element{
    fn format(&self) -> AudioFrameFormat;
    fn on_pull(&mut self, pipline: &AudioPipeline) -> Result<Option<AudioFrame>, Error>;
}

pub trait AudioSink: Element{
    fn add_source(&mut self, format: AudioFrameFormat) -> Result<(), Error>;
    fn on_push(&mut self, pipline: &AudioPipeline, frame: AudioFrame) -> Result<(), Error>;
}

pub trait SubtitleSource: Element{
    fn format(&self) -> SubtitleFrameFormat;
    fn on_pull(&mut self, pipline: &SubtitlePipeline) -> Result<Option<SubtitleFrame>, Error>;
}

pub trait SubtitleSink: Element{
    fn add_source(&mut self, format: SubtitleFrameFormat) -> Result<(), Error>;
    fn on_push(&mut self, pipline: &SubtitlePipeline, frame: SubtitleFrame) -> Result<(), Error>;
}


pub trait Demuxer{
    fn next_video_packet(&mut self) -> Option<VideoPacket>;
    fn next_audio_packet(&mut self) -> Option<AudioPacket>;
    fn next_subtitle_packet(&mut self) -> Option<SubtitlePacket>;
}

pub trait Element{
    fn name(&self) -> &'static str{
        return core::any::type_name::<Self>()
    }
    fn as_video_decoder(&mut self) -> Option<&mut dyn VideoDecoder>{
        None
    }
    fn as_audio_decoder(&mut self) -> Option<&mut dyn AudioDecoder>{
        None
    }
    fn as_subtitle_decoder(&mut self) -> Option<&mut dyn SubtitleDecoder>{
        None
    }

    fn as_video_source(&mut self) -> Option<&mut dyn VideoSource>{
        None
    }
    fn as_audio_source(&mut self) -> Option<&mut dyn AudioSource>{
        None
    }
    fn as_subtitle_source(&mut self) -> Option<&mut dyn SubtitleSource>{
        None
    }

    fn as_video_sink(&mut self) -> Option<&mut dyn VideoSink>{
        None
    }
    fn as_audio_sink(&mut self) -> Option<&mut dyn AudioSink>{
        None
    }
    fn as_subtitle_sink(&mut self) -> Option<&mut dyn SubtitleSink>{
        None
    }
}