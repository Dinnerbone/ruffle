use crate::{
    BlurFilter, BlurFilterFlags, Color, Fixed16, Fixed8, GlowFilter, GlowFilterFlags, Rectangle,
};
use bitflags::bitflags;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropShadowFilter {
    pub color: Color,
    pub blur_x: Fixed16,
    pub blur_y: Fixed16,
    pub angle: Fixed16,
    pub distance: Fixed16,
    pub strength: Fixed8,
    pub flags: DropShadowFilterFlags,
}

impl DropShadowFilter {
    #[inline]
    pub fn is_inner(&self) -> bool {
        self.flags.contains(DropShadowFilterFlags::INNER_SHADOW)
    }

    #[inline]
    pub fn is_knockout(&self) -> bool {
        self.flags.contains(DropShadowFilterFlags::KNOCKOUT)
    }

    #[inline]
    pub fn num_passes(&self) -> u8 {
        (self.flags & DropShadowFilterFlags::PASSES).bits()
    }

    #[inline]
    pub fn hide_object(&self) -> bool {
        !self.flags.contains(DropShadowFilterFlags::COMPOSITE_SOURCE)
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.blur_x *= Fixed16::from_f32(x);
        self.blur_y *= Fixed16::from_f32(y);
        self.distance *= Fixed16::from_f32(y);
    }

    pub fn inner_blur_filter(&self) -> BlurFilter {
        BlurFilter {
            blur_x: self.blur_x,
            blur_y: self.blur_y,
            flags: BlurFilterFlags::from_passes(self.num_passes()),
        }
    }

    pub fn inner_glow_filter(&self) -> GlowFilter {
        let mut flags = GlowFilterFlags::from_passes(self.num_passes());
        flags.set(GlowFilterFlags::INNER_GLOW, self.is_inner());
        flags.set(GlowFilterFlags::KNOCKOUT, self.is_knockout());
        flags.set(GlowFilterFlags::COMPOSITE_SOURCE, !self.hide_object());
        GlowFilter {
            color: self.color,
            blur_x: self.blur_x,
            blur_y: self.blur_y,
            strength: self.strength,
            flags,
        }
    }

    pub fn calculate_dest_rect(&self, source_rect: Rectangle<i32>) -> Rectangle<i32> {
        let mut result = self.inner_glow_filter().calculate_dest_rect(source_rect);
        let distance = self.distance.to_f32();
        let angle = self.angle.to_f32();
        let x = (angle.cos() * distance).ceil() as i32;
        let y = (angle.sin() * distance).ceil() as i32;
        if x < 0 {
            result.x_min += x;
        } else {
            result.x_max += x;
        }
        if y < 0 {
            result.y_min += y;
        } else {
            result.y_max += y;
        }
        result
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct DropShadowFilterFlags: u8 {
        const INNER_SHADOW     = 1 << 7;
        const KNOCKOUT         = 1 << 6;
        const COMPOSITE_SOURCE = 1 << 5;
        const PASSES           = 0b11111;
    }
}

impl DropShadowFilterFlags {
    #[inline]
    pub fn from_passes(num_passes: u8) -> Self {
        let flags = Self::from_bits_retain(num_passes);
        debug_assert_eq!(flags & Self::PASSES, flags);
        flags
    }
}
