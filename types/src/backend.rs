use crate::backend::audio::AudioBackend;
use crate::backend::log::LogBackend;
use crate::backend::navigator::NavigatorBackend;
use crate::backend::render::RenderBackend;
use crate::backend::storage::StorageBackend;
use crate::backend::ui::UiBackend;
use crate::backend::video::VideoBackend;

pub mod audio;
pub mod log;
pub mod navigator;
pub mod render;
pub mod storage;
pub mod ui;
pub mod video;

pub trait Backend {
    type Audio: AudioBackend;
    type Log: LogBackend;
    type Navigator: NavigatorBackend;
    type Renderer: RenderBackend;
    type Storage: StorageBackend;
    type Ui: UiBackend;
    type Video: VideoBackend;

    fn audio(&self) -> &Self::Audio;
    fn audio_mut(&mut self) -> &mut Self::Audio;

    fn log(&self) -> &Self::Log;
    fn log_mut(&mut self) -> &mut Self::Log;

    fn navigator(&self) -> &Self::Navigator;
    fn navigator_mut(&mut self) -> &mut Self::Navigator;

    fn renderer(&self) -> &Self::Renderer;
    fn renderer_mut(&mut self) -> &mut Self::Renderer;

    fn storage(&self) -> &Self::Storage;
    fn storage_mut(&mut self) -> &mut Self::Storage;

    fn ui(&self) -> &Self::Ui;
    fn ui_mut(&mut self) -> &mut Self::Ui;

    fn video(&self) -> &Self::Video;
    fn video_mut(&mut self) -> &mut Self::Video;
}
