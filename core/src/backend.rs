use crate::backend::input::InputBackend;
use crate::backend::render::RenderBackend;
use crate::backend::navigator::NavigatorBackend;
use crate::backend::audio::AudioBackend;
use std::fmt::Debug;

pub mod audio;
pub mod input;
pub mod navigator;
pub mod render;

pub trait Backends: Debug + 'static {
    type Audio: AudioBackend;
    type Navigator: NavigatorBackend;
    type Renderer: RenderBackend;
    type Input: InputBackend;
}