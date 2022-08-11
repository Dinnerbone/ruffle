use crate::add_field_accessors;
use crate::avm1::{Object, ScriptObject, TObject};
use crate::context::UpdateContext;
use crate::impl_custom_object;
use gc_arena::{Collect, GcCell, MutationContext};

use crate::bitmap::bitmap_data::BitmapData;
use ruffle_types::backend::Backend;
use std::fmt;

/// A BitmapData
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct BitmapDataObject<'gc, B: Backend>(GcCell<'gc, BitmapDataData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct BitmapDataData<'gc, B: Backend> {
    /// The underlying script object.
    base: ScriptObject<'gc, B>,
    data: GcCell<'gc, BitmapData<'gc, B>>,
    disposed: bool,
}

impl<B: Backend> fmt::Debug for BitmapDataObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("BitmapData")
            .field("data", &this.data)
            .finish()
    }
}

impl<'gc, B: Backend> BitmapDataObject<'gc, B> {
    add_field_accessors!(
        [disposed, bool, get => disposed],
        [data, GcCell<'gc, BitmapData<'gc, B>>, set => set_bitmap_data, get => bitmap_data],
    );

    pub fn empty_object(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> Self {
        Self::with_bitmap_data(gc_context, proto, Default::default())
    }

    pub fn with_bitmap_data(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
        bitmap_data: BitmapData<'gc, B>,
    ) -> Self {
        Self(GcCell::allocate(
            gc_context,
            BitmapDataData {
                base: ScriptObject::object(gc_context, proto),
                disposed: false,
                data: GcCell::allocate(gc_context, bitmap_data),
            },
        ))
    }

    pub fn dispose(&self, context: &mut UpdateContext<'_, 'gc, '_, B>) {
        self.bitmap_data()
            .write(context.gc_context)
            .dispose(context.renderer);
        self.0.write(context.gc_context).disposed = true;
    }
}

impl<'gc, B: Backend> TObject<'gc> for BitmapDataObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base {
        bare_object(as_bitmap_data_object -> BitmapDataObject::empty_object);
    });
}
