use std::borrow::Cow;
use std::sync::Arc;

use crate::backend::{RenderBackend, ShapeHandle, ViewportDimensions};
use crate::bitmap::{Bitmap, BitmapHandle, BitmapHandleImpl, BitmapSize, BitmapSource, SyncHandle};
use crate::commands::CommandList;
use crate::error::Error;
use crate::matrix::Matrix;
use crate::quality::StageQuality;
use crate::shape_utils::{ShapeFills, ShapeStrokes};
use gc_arena::MutationContext;
use swf::{CharacterId, Color};

use super::{Context3D, Context3DCommand};

pub struct NullBitmapSource;

impl BitmapSource for NullBitmapSource {
    fn bitmap_size(&self, _id: u16) -> Option<BitmapSize> {
        None
    }
    fn bitmap_handle(&self, _id: u16, _renderer: &mut dyn RenderBackend) -> Option<BitmapHandle> {
        None
    }
}

pub struct NullRenderer {
    dimensions: ViewportDimensions,
}

impl NullRenderer {
    pub fn new(dimensions: ViewportDimensions) -> Self {
        Self { dimensions }
    }
}
#[derive(Clone, Debug)]
struct NullBitmapHandle;
impl BitmapHandleImpl for NullBitmapHandle {}

impl RenderBackend for NullRenderer {
    fn viewport_dimensions(&self) -> ViewportDimensions {
        self.dimensions
    }
    fn set_viewport_dimensions(&mut self, dimensions: ViewportDimensions) {
        self.dimensions = dimensions;
    }
    fn register_shape_fills(&mut self, _shape: &ShapeFills, _id: CharacterId) -> ShapeHandle {
        ShapeHandle(0)
    }
    fn replace_shape_fills(&mut self, _shape: &ShapeFills, _id: CharacterId, _handle: ShapeHandle) {
    }
    fn register_shape_strokes(
        &mut self,
        _shape: &ShapeStrokes,
        _id: CharacterId,
        _matrix: Matrix,
    ) -> ShapeHandle {
        ShapeHandle(0)
    }
    fn replace_shape_strokes(
        &mut self,
        _shape: &ShapeStrokes,
        _id: CharacterId,
        _matrix: Matrix,
        _handle: ShapeHandle,
    ) {
    }
    fn register_glyph_shape(&mut self, _shape: &swf::Glyph) -> ShapeHandle {
        ShapeHandle(0)
    }

    fn render_offscreen(
        &mut self,
        _handle: BitmapHandle,
        _width: u32,
        _height: u32,
        _commands: CommandList,
        _quality: StageQuality,
    ) -> Option<Box<dyn SyncHandle>> {
        None
    }

    fn submit_frame(&mut self, _clear: Color, _commands: CommandList) {}
    fn register_bitmap(&mut self, _bitmap: Bitmap) -> Result<BitmapHandle, Error> {
        Ok(BitmapHandle(Arc::new(NullBitmapHandle)))
    }

    fn update_texture(
        &mut self,
        _bitmap: &BitmapHandle,
        _width: u32,
        _height: u32,
        _rgba: Vec<u8>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn create_context3d(&mut self) -> Result<Box<dyn super::Context3D>, Error> {
        Err(Error::Unimplemented("createContext3D".into()))
    }

    fn context3d_present<'gc>(
        &mut self,
        _context: &mut dyn Context3D,
        _commands: Vec<Context3DCommand<'gc>>,
        _mc: MutationContext<'gc, '_>,
    ) -> Result<(), Error> {
        Err(Error::Unimplemented("Context3D.present".into()))
    }

    fn debug_info(&self) -> Cow<'static, str> {
        Cow::Borrowed("Renderer: Null")
    }

    fn set_quality(&mut self, _quality: StageQuality) {}
}
