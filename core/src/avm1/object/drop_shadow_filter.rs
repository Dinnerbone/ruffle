use crate::add_field_accessors;
use crate::avm1::{Object, ScriptObject, TObject};
use crate::impl_custom_object;
use gc_arena::{Collect, GcCell, MutationContext};

use ruffle_types::backend::Backend;
use std::fmt;

/// A DropShadowFilter
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct DropShadowFilterObject<'gc, B: Backend>(GcCell<'gc, DropShadowFilterData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct DropShadowFilterData<'gc, B: Backend> {
    /// The underlying script object.
    base: ScriptObject<'gc, B>,

    alpha: f64,
    angle: f64,
    blur_x: f64,
    blur_y: f64,
    color: u32,
    distance: f64,
    hide_object: bool,
    inner: bool,
    knockout: bool,
    quality: i32,
    strength: f64,
}

impl<B: Backend> fmt::Debug for DropShadowFilterObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("DropShadowFilter")
            .field("alpha", &this.alpha)
            .field("angle", &this.angle)
            .field("blurX", &this.blur_x)
            .field("blurY", &this.blur_y)
            .field("color", &this.color)
            .field("distance", &this.distance)
            .field("hide_object", &this.hide_object)
            .field("inner", &this.inner)
            .field("knockout", &this.knockout)
            .field("quality", &this.quality)
            .field("strength", &this.strength)
            .finish()
    }
}

impl<'gc, B: Backend> DropShadowFilterObject<'gc, B> {
    add_field_accessors!(
        [set_alpha, alpha, alpha, f64],
        [set_angle, angle, angle, f64],
        [set_blur_x, blur_x, blur_x, f64],
        [set_blur_y, blur_y, blur_y, f64],
        [set_color, color, color, u32],
        [set_hide_object, hide_object, hide_object, bool],
        [set_distance, distance, distance, f64],
        [set_inner, inner, inner, bool],
        [set_knockout, knockout, knockout, bool],
        [set_quality, quality, quality, i32],
        [set_strength, strength, strength, f64],
    );

    pub fn empty_object(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> Self {
        DropShadowFilterObject(GcCell::allocate(
            gc_context,
            DropShadowFilterData {
                base: ScriptObject::object(gc_context, proto),
                distance: 4.0,
                hide_object: false,
                angle: 44.9999999772279,
                color: 0x000000,
                alpha: 1.0,
                blur_x: 4.0,
                blur_y: 4.0,
                strength: 1.0,
                quality: 1,
                inner: false,
                knockout: false,
            },
        ))
    }
}

impl<'gc, B: Backend> TObject<'gc> for DropShadowFilterObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base {
        bare_object(as_drop_shadow_filter_object -> DropShadowFilterObject::empty_object);
    });
}
