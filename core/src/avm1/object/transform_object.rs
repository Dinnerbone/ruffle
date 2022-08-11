use crate::avm1::{Object, ScriptObject, TObject};
use crate::display_object::MovieClip;
use crate::impl_custom_object;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::fmt;

/// A flash.geom.Transform object
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct TransformObject<'gc, B: Backend>(GcCell<'gc, TransformData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct TransformData<'gc, B: Backend> {
    /// The underlying script object.
    base: ScriptObject<'gc, B>,
    clip: Option<MovieClip<'gc, B>>,
}

impl<B: Backend> fmt::Debug for TransformObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("Transform")
            .field("clip", &this.clip)
            .finish()
    }
}

impl<'gc, B: Backend> TransformObject<'gc, B> {
    pub fn empty(gc_context: MutationContext<'gc, '_>, proto: Option<Object<'gc, B>>) -> Self {
        Self(GcCell::allocate(
            gc_context,
            TransformData {
                base: ScriptObject::object(gc_context, proto),
                clip: None,
            },
        ))
    }

    pub fn clip(self) -> Option<MovieClip<'gc, B>> {
        self.0.read().clip
    }

    pub fn set_clip(self, gc_context: MutationContext<'gc, '_>, clip: MovieClip<'gc, B>) {
        self.0.write(gc_context).clip = Some(clip)
    }
}

impl<'gc, B: Backend> TObject<'gc> for TransformObject<'gc, B> {
    type B = B;
    impl_custom_object!(B, base {
        bare_object(as_transform_object -> TransformObject::empty);
    });
}
