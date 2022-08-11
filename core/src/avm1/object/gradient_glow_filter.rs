use crate::add_field_accessors;
use crate::avm1::{Object, ScriptObject, TObject};
use crate::impl_custom_object;
use gc_arena::{Collect, GcCell, MutationContext};

use crate::avm1::object::bevel_filter::BevelFilterType;
use ruffle_types::backend::Backend;
use std::fmt;

/// A GradientGlowFilter
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct GradientGlowFilterObject<'gc, B: Backend>(GcCell<'gc, GradientGlowFilterData<'gc, B>>);

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct GradientGlowFilterData<'gc, B: Backend> {
    /// The underlying script object.
    base: ScriptObject<'gc, B>,

    alphas: Vec<f64>,
    angle: f64,
    blur_x: f64,
    blur_y: f64,
    colors: Vec<u32>,
    distance: f64,
    knockout: bool,
    quality: i32,
    ratios: Vec<u8>,
    strength: f64,
    type_: BevelFilterType,
}

impl<B: Backend> fmt::Debug for GradientGlowFilterObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("GradientGlowFilter")
            .field("alphas", &this.alphas)
            .field("angle", &this.angle)
            .field("blurX", &this.blur_x)
            .field("blurY", &this.blur_y)
            .field("colors", &this.colors)
            .field("distance", &this.distance)
            .field("knockout", &this.knockout)
            .field("quality", &this.quality)
            .field("ratios", &this.ratios)
            .field("strength", &this.strength)
            .field("type_", &this.type_)
            .finish()
    }
}

impl<'gc, B: Backend> GradientGlowFilterObject<'gc, B> {
    add_field_accessors!(
        [angle, f64, set => set_angle, get => angle],
        [blur_x, f64, set => set_blur_x, get => blur_x],
        [blur_y, f64, set => set_blur_y, get => blur_y],
        [distance, f64, set => set_distance, get => distance],
        [knockout, bool, set => set_knockout, get => knockout],
        [quality, i32, set => set_quality, get => quality],
        [strength, f64, set => set_strength, get => strength],
        [type_, BevelFilterType, set => set_type, get => get_type],
        [alphas, Vec<f64>, set => set_alphas],
        [colors, Vec<u32>, set => set_colors],
        [ratios, Vec<u8>, set => set_ratios],
    );

    pub fn alphas(&self) -> Vec<f64> {
        self.0.read().alphas.clone()
    }

    pub fn colors(&self) -> Vec<u32> {
        self.0.read().colors.clone()
    }

    pub fn ratios(&self) -> Vec<u8> {
        self.0.read().ratios.clone()
    }

    pub fn empty_object(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> Self {
        GradientGlowFilterObject(GcCell::allocate(
            gc_context,
            GradientGlowFilterData {
                base: ScriptObject::object(gc_context, proto),
                alphas: vec![],
                angle: 0.0,
                blur_x: 0.0,
                blur_y: 0.0,
                colors: vec![],
                distance: 0.0,
                knockout: false,
                quality: 0,
                ratios: vec![],
                strength: 0.0,
                type_: BevelFilterType::Inner,
            },
        ))
    }
}

impl<'gc, B: Backend> TObject<'gc> for GradientGlowFilterObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base {
        bare_object(as_gradient_glow_filter_object -> GradientGlowFilterObject::empty_object);
    });
}
