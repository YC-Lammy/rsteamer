use core::marker::PhantomData;
use core::sync::atomic::{
    Ordering,
    AtomicU8
};
use std::sync::Arc;

use crate::source::{VideoFrame, AudioFrame, SubtitleFrame, Element};
use crate::source::{VideoSink, VideoSource,AudioSink, SubtitleSink, AudioSource, SubtitleSource};
use crate::error::Error;

pub struct ElementID(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Link{
    Video(usize, usize),
    Audio(usize, usize),
    Subtitle(usize, usize),
}

pub struct ElementRef<'a, T>{
    loc: usize,
    _m: PhantomData<&'a T>
}

pub struct PiplineBuilder{
    elements: Vec<Box<dyn Element>>,
    links: Vec<Link>,
}

impl PiplineBuilder{
    pub const fn new() -> Self{
        Self { 
            elements: Vec::new(),
            links: Vec::new()
        }
    }

    fn check_link(&self, link: Link) -> Result<(), Error>{
        if self.links.contains(&link){
            return Err(Error::DuplicatedLink)
        }
        todo!()
    }

    pub fn add_element<'a, T:Element + 'static>(&'a mut self, elem: T) -> ElementRef<'a, T>{
        let id = self.elements.len();

        self.elements.push(Box::new(elem));

        return ElementRef { loc: id, _m: PhantomData }
    }

    pub fn link_video<'a, SRC:VideoSource + 'static, SINK:VideoSink + 'static>(&mut self, source: ElementRef<'a, SRC>, sink: ElementRef<'a, SINK>) -> Result<(), Error>{
        let link = Link::Video(source.loc, sink.loc);
        self.check_link(link)?;
        self.links.push(link);

        return Ok(())
    }

    pub fn link_audio<'a, SRC:AudioSource + 'static, SINK:AudioSink + 'static>(&mut self, source: ElementRef<'a, SRC>, sink: ElementRef<'a, SINK>) -> Result<(), Error>{
        let link = Link::Audio(source.loc, sink.loc);
        self.check_link(link)?;
        self.links.push(link);

        return Ok(())
    }

    pub fn link_subtitle<'a, SRC:SubtitleSource + 'static, SINK:SubtitleSink + 'static>(&mut self, source: ElementRef<'a, SRC>, sink: ElementRef<'a, SINK>) -> Result<(), Error>{
        let link = Link::Subtitle(source.loc, sink.loc);
        self.check_link(link)?;
        self.links.push(link);

        return Ok(())
    }
}

#[repr(u8)]
pub enum PiplineState{
    Null,
    Ready,
    Pause,
    Play
}

// a pipeline represents the pipline as a whole.
pub struct Pipeline{
    state: AtomicU8
}

impl Pipeline{
    pub fn pause(&self){
        self.state.store(PiplineState::Pause as u8, Ordering::SeqCst);
    }
    pub fn play(&self){
        self.state.store(PiplineState::Play as u8, Ordering::SeqCst);
    }

    pub fn is_paused(&self) -> bool{
        self.state.load(Ordering::SeqCst) == PiplineState::Pause as u8
    }

    pub fn is_playing(&self) -> bool{
        self.state.load(Ordering::SeqCst) == PiplineState::Play as u8
    }

    pub fn state(&self) -> PiplineState{
        unsafe{core::mem::transmute(self.state.load(Ordering::SeqCst))}
    }
}

/// a video pipline represents a video branch in the pipline.
/// It may or may not own a thread
pub struct VideoPipeline{
    pipeline: Arc<Pipeline>,
}

impl VideoPipeline{
    pub fn pause(&self){
        self.pipeline.pause();
    }
    pub fn play(&self){
        self.pipeline.play();
    }
    pub fn is_paused(&self) -> bool{
        self.pipeline.is_paused()
    }
    pub fn is_playing(&self) -> bool{
        self.pipeline.is_playing()
    }
    pub fn state(&self) -> PiplineState{
        self.pipeline.state()
    }

    pub fn pull_frame(&self) -> Option<VideoFrame>{
        todo!()
    }
    pub fn push_frame(&self, frame: VideoFrame){

    }
}

pub struct AudioPipeline{
    pipeline: Arc<Pipeline>
}

impl AudioPipeline{
    pub fn pause(&self){
        self.pipeline.pause();
    }
    pub fn play(&self){
        self.pipeline.play();
    }
    pub fn is_paused(&self) -> bool{
        self.pipeline.is_paused()
    }
    pub fn is_playing(&self) -> bool{
        self.pipeline.is_playing()
    }
    pub fn state(&self) -> PiplineState{
        self.pipeline.state()
    }

    pub fn pull_frame(&self) -> Option<AudioFrame>{
        todo!()
    }
    pub fn push_frame(&self, frame: AudioFrame){

    }
}

pub struct SubtitlePipeline{
    pipeline: Arc<Pipeline>
}

impl SubtitlePipeline{
    pub fn pause(&self){
        self.pipeline.pause();
    }
    pub fn play(&self){
        self.pipeline.play();
    }
    pub fn is_paused(&self) -> bool{
        self.pipeline.is_paused()
    }
    pub fn is_playing(&self) -> bool{
        self.pipeline.is_playing()
    }
    pub fn state(&self) -> PiplineState{
        self.pipeline.state()
    }

    pub fn pull_frame(&self) -> Option<SubtitleFrame>{
        todo!()
    }
    pub fn push_frame(&self, frame: SubtitleFrame){

    }
}