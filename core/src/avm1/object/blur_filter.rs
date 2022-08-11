use crate::add_field_accessors;
use crate::avm1::{Object, ScriptObject, TObject};
use crate::impl_custom_object;
use gc_arena::{Collect, GcCell, MutationContext};

use ruffle_types::backend::Backend;
use std::fmt;

/// A BlurFilter
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct BlurFilterObject<'gc, B: Backend>(GcCell<'gc, BlurFilterData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct BlurFilterData<'gc, B: Backend> {
    /// The underlying script object.
    base: ScriptObject<'gc, B>,

    blur_x: f64,
    blur_y: f64,
    quality: i32,
}

impl<B: Backend> fmt::Debug for BlurFilterObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("BlurFilter")
            .field("blurX", &this.blur_x)
            .field("blurY", &this.blur_y)
            .field("quality", &this.quality)
            .finish()
    }
}

impl<'gc, B: Backend> BlurFilterObject<'gc, B> {
    add_field_accessors!(
        [set_blur_x, blur_x, blur_x, f64],
        [set_blur_y, blur_y, blur_y, f64],
        [set_quality, quality, quality, i32],
    );

    pub fn empty_object(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> Self {
        BlurFilterObject(GcCell::allocate(
            gc_context,
            BlurFilterData {
                base: ScriptObject::object(gc_context, proto),
                blur_x: 4.0,
                blur_y: 4.0,
                quality: 1,
            },
        ))
    }
}

impl<'gc, B: Backend> TObject<'gc> for BlurFilterObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base {
        bare_object(as_blur_filter_object -> BlurFilterObject::empty_object);
    });
}
