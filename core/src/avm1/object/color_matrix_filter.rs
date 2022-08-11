use crate::add_field_accessors;
use crate::avm1::{Object, ScriptObject, TObject};
use crate::impl_custom_object;
use gc_arena::{Collect, GcCell, MutationContext};

use ruffle_types::backend::Backend;
use std::fmt;

/// A ColorMatrixFilter
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct ColorMatrixFilterObject<'gc, B: Backend>(GcCell<'gc, ColorMatrixFilterData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct ColorMatrixFilterData<'gc, B: Backend> {
    /// The underlying script object.
    base: ScriptObject<'gc, B>,

    matrix: [f64; 4 * 5],
}

impl<B: Backend> fmt::Debug for ColorMatrixFilterObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("ColorMatrixFilter")
            .field("matrix", &this.matrix)
            .finish()
    }
}

impl<'gc, B: Backend> ColorMatrixFilterObject<'gc, B> {
    add_field_accessors!([set_matrix, matrix, matrix, [f64; 4 * 5]],);

    pub fn empty_object(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> Self {
        ColorMatrixFilterObject(GcCell::allocate(
            gc_context,
            ColorMatrixFilterData {
                base: ScriptObject::object(gc_context, proto),
                matrix: [
                    1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                ],
            },
        ))
    }
}

impl<'gc, B: Backend> TObject<'gc> for ColorMatrixFilterObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base {
        bare_object(as_color_matrix_filter_object -> ColorMatrixFilterObject::empty_object);
    });
}
