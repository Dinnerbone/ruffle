use crate::avm1::Avm1;
use crate::backend::render::ShapeHandle;
use crate::context::{RenderContext, UpdateContext};
use crate::display_object::{DisplayObjectBase, TDisplayObject};
use crate::prelude::*;
use gc_arena::{Collect, GcCell};
use crate::backend::Backends;

#[derive(Clone, Debug, Collect, Copy)]
#[collect(no_drop)]
pub struct Graphic<'gc, B: Backends>(GcCell<'gc, GraphicData<'gc, B>>);

#[derive(Clone, Debug)]
pub struct GraphicData<'gc, B: Backends> {
    base: DisplayObjectBase<'gc, B>,
    static_data: gc_arena::Gc<'gc, GraphicStatic>,
}

impl<'gc, B: Backends> Graphic<'gc, B> {
    pub fn from_swf_tag(context: &mut UpdateContext<'_, 'gc, '_, B>, swf_shape: &swf::Shape) -> Self {
        let static_data = GraphicStatic {
            id: swf_shape.id,
            render_handle: context.renderer.register_shape(swf_shape),
            bounds: swf_shape.shape_bounds.clone().into(),
        };
        Graphic(GcCell::allocate(
            context.gc_context,
            GraphicData {
                base: Default::default(),
                static_data: gc_arena::Gc::allocate(context.gc_context, static_data),
            },
        ))
    }
}

impl<'gc, B: Backends> TDisplayObject<'gc, B> for Graphic<'gc, B> {
    impl_display_object!(base);

    fn id(&self) -> CharacterId {
        self.0.read().static_data.id
    }

    fn self_bounds(&self) -> BoundingBox {
        self.0.read().static_data.bounds.clone()
    }

    fn world_bounds(&self) -> BoundingBox {
        // TODO: Use dirty flags and cache this.
        let mut bounds = self.local_bounds();
        let mut node = self.parent();
        while let Some(display_object) = node {
            bounds = bounds.transform(&*display_object.matrix());
            node = display_object.parent();
        }
        bounds
    }

    fn run_frame(&mut self, _avm: &mut Avm1<'gc, B>, _context: &mut UpdateContext<B>) {
        // Noop
    }

    fn render(&self, context: &mut RenderContext<B>) {
        if !self.world_bounds().intersects(&context.view_bounds) {
            // Off-screen; culled
            return;
        }

        context.transform_stack.push(&*self.transform());

        context.renderer.render_shape(
            self.0.read().static_data.render_handle,
            context.transform_stack.transform(),
        );

        context.transform_stack.pop();
    }
}

unsafe impl<'gc, B: Backends> gc_arena::Collect for GraphicData<'gc, B> {
    fn trace(&self, cc: gc_arena::CollectionContext) {
        self.base.trace(cc);
        self.static_data.trace(cc);
    }
}

/// Static data shared between all instances of a graphic.
#[allow(dead_code)]
struct GraphicStatic {
    id: CharacterId,
    render_handle: ShapeHandle,
    bounds: BoundingBox,
}

unsafe impl<'gc> gc_arena::Collect for GraphicStatic {
    #[inline]
    fn needs_trace() -> bool {
        false
    }
}
