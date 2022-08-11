//! `flash.display.Stage` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{Object, TObject};
use crate::avm2::traits::Trait;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::TDisplayObject;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;
use ruffle_types::display_object::stage::StageDisplayState;
use ruffle_types::string::{AvmString, WString};
use swf::Color;

/// Implements `flash.display.Stage`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("You cannot construct new instances of the Stage.".into())
}

/// Implements `flash.display.Stage`'s native instance constructor.
pub fn native_instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, args)?;
    }

    Ok(Value::Undefined)
}

/// Implements `flash.display.Stage`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Overrides `accessibilityProperties`'s setter.
pub fn set_accessibility_properties<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set accessibility properties on the stage.".into())
}

/// Overrides `alpha`'s setter.
pub fn set_alpha<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage's opacity.".into())
}

/// Overrides `blendMode`'s setter.
pub fn set_blend_mode<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the blend mode of the stage.".into())
}

/// Overrides `cacheAsBitmap`'s setter.
pub fn set_cache_as_bitmap<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage to be cached as a bitmap.".into())
}

/// Overrides `contextMenu`'s setter.
pub fn set_context_menu<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage's context menu.".into())
}

/// Overrides `filters`'s setter.
pub fn set_filters<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot apply filters to the stage.".into())
}

/// Overrides `focusRect`'s setter.
pub fn set_focus_rect<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage's focus rect.".into())
}

/// Overrides `loaderInfo`'s setter.
pub fn set_loader_info<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the blend mode of the stage.".into())
}

/// Overrides `mask`'s setter.
pub fn set_mask<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot mask the stage.".into())
}

/// Overrides `mouseEnabled`'s setter.
pub fn set_mouse_enabled<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot enable or disable the mouse on the stage.".into())
}

/// Overrides `name`'s getter.
pub fn name<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Null)
}

/// Overrides `name`'s setter.
pub fn set_name<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the name of the stage.".into())
}

/// Overrides `opaqueBackground`'s setter.
pub fn set_opaque_background<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot give or take away the stage's opaque background.".into())
}

/// Overrides `rotation`'s setter.
pub fn set_rotation<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot rotate the stage.".into())
}

/// Overrides `scale9Grid`'s setter.
pub fn set_scale_nine_grid<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage's 9-slice grid.".into())
}

/// Overrides `scaleX`'s setter.
pub fn set_scale_x<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage's horizontal scale.".into())
}

/// Overrides `scaleY`'s setter.
pub fn set_scale_y<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage's vertical scale.".into())
}

/// Overrides `scrollRect`'s setter.
pub fn set_scroll_rect<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage's scroll rectangle.".into())
}

/// Overrides `tabEnabled`'s setter.
pub fn set_tab_enabled<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot enable or disable tabbing the stage.".into())
}

/// Overrides `tabIndex`'s setter.
pub fn set_tab_index<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot set the stage's tab index.".into())
}

/// Overrides `transform`'s setter.
pub fn set_transform<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot transform the stage.".into())
}

/// Overrides `visible`'s setter.
pub fn set_visible<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot hide or unhide the stage.".into())
}

/// Overrides `x`'s setter.
pub fn set_x<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot move the stage horizontally.".into())
}

/// Overrides `y`'s setter.
pub fn set_y<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Error: You cannot move the stage vertically.".into())
}

/// Implement `align`'s getter
pub fn align<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let align = activation.context.stage.align();
    let mut s = WString::with_capacity(4, false);
    // Match string values returned by AS.
    // It's possible to have an oxymoronic "TBLR".
    // This acts the same as "TL" (top-left takes priority).
    // This order is different between AVM1 and AVM2!
    use crate::display_object::StageAlign;
    if align.contains(StageAlign::TOP) {
        s.push_byte(b'T');
    }
    if align.contains(StageAlign::BOTTOM) {
        s.push_byte(b'B');
    }
    if align.contains(StageAlign::LEFT) {
        s.push_byte(b'L');
    }
    if align.contains(StageAlign::RIGHT) {
        s.push_byte(b'R');
    }
    let align = AvmString::new(activation.context.gc_context, s);
    Ok(align.into())
}

/// Implement `align`'s setter
pub fn set_align<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let align = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_string(activation)?
        .parse()
        .unwrap_or_default();
    activation
        .context
        .stage
        .set_align(&mut activation.context, align);
    Ok(Value::Undefined)
}

/// Implement `browserZoomFactor`'s getter
pub fn browser_zoom_factor<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        return Ok(dobj.viewport_scale_factor().into());
    }

    Ok(Value::Undefined)
}

/// Implement `color`'s getter
pub fn color<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        let color = dobj.background_color().unwrap_or(Color::WHITE);
        return Ok(color.to_rgba().into());
    }

    Ok(Value::Undefined)
}

/// Implement `color`'s setter
pub fn set_color<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        let color = Color::from_rgb(
            args.get(0)
                .cloned()
                .unwrap_or(Value::Undefined)
                .coerce_to_u32(activation)?,
            255,
        );
        dobj.set_background_color(activation.context.gc_context, Some(color));
    }

    Ok(Value::Undefined)
}

/// Implement `contentsScaleFactor`'s getter
pub fn contents_scale_factor<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        return Ok(dobj.viewport_scale_factor().into());
    }

    Ok(Value::Undefined)
}

/// Implement `displayState`'s getter
pub fn display_state<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let display_state = AvmString::new_utf8(
        activation.context.gc_context,
        activation.context.stage.display_state().to_string(),
    );
    Ok(display_state.into())
}

/// Implement `displayState`'s setter
pub fn set_display_state<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Ok(mut display_state) = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_string(activation)?
        .parse()
    {
        // It's not entirely clear why when setting to FullScreen, desktop flash player at least will
        // set its value to FullScreenInteractive. Overriding until flash logic is clearer.
        if display_state == StageDisplayState::FullScreen {
            display_state = StageDisplayState::FullScreenInteractive;
        }
        activation
            .context
            .stage
            .set_display_state(&mut activation.context, display_state);
    } else {
        return Err(
            "ArgumentError: Error #2008: Parameter displayState must be one of the accepted values."
                .into(),
        );
    }
    Ok(Value::Undefined)
}

/// Implement `focus`'s getter
pub fn focus<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(activation
        .context
        .focus_tracker
        .get()
        .and_then(|focus_dobj| focus_dobj.object2().as_object())
        .map(|o| o.into())
        .unwrap_or(Value::Null))
}

/// Implement `focus`'s setter
pub fn set_focus<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let focus = activation.context.focus_tracker;
    match args.get(0).cloned().unwrap_or(Value::Undefined) {
        Value::Null => focus.set(None, &mut activation.context),
        val => {
            if let Some(dobj) = val.as_object().and_then(|o| o.as_display_object()) {
                focus.set(Some(dobj), &mut activation.context);
            } else {
                return Err("Cannot set focus to non-DisplayObject".into());
            }
        }
    };

    Ok(Value::Undefined)
}

/// Implement `frameRate`'s getter
pub fn frame_rate<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok((*activation.context.frame_rate).into())
}

/// Implement `frameRate`'s setter
pub fn set_frame_rate<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let new_frame_rate = args
        .get(0)
        .cloned()
        .unwrap_or(Value::Undefined)
        .coerce_to_number(activation)?;
    *activation.context.frame_rate = new_frame_rate;

    Ok(Value::Undefined)
}

pub fn show_default_context_menu<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(activation.context.stage.show_menu().into())
}

pub fn set_show_default_context_menu<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let show_default_context_menu = args.get(0).unwrap_or(&Value::Undefined).coerce_to_boolean();
    activation
        .context
        .stage
        .set_show_menu(&mut activation.context, show_default_context_menu);
    Ok(Value::Undefined)
}

/// Implement `scaleMode`'s getter
pub fn scale_mode<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let scale_mode = AvmString::new_utf8(
        activation.context.gc_context,
        activation.context.stage.scale_mode().to_string(),
    );
    Ok(scale_mode.into())
}

/// Implement `scaleMode`'s setter
pub fn set_scale_mode<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Ok(scale_mode) = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_string(activation)?
        .parse()
    {
        activation
            .context
            .stage
            .set_scale_mode(&mut activation.context, scale_mode);
    } else {
        return Err(
            "ArgumentError: Error #2008: Parameter scaleMode must be one of the accepted values."
                .into(),
        );
    }
    Ok(Value::Undefined)
}

/// Implement `stageWidth`'s getter
pub fn stage_width<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        return Ok(dobj.stage_size().0.into());
    }

    Ok(Value::Undefined)
}

/// Implement `stageWidth`'s setter
pub fn set_stage_width<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    // For some reason this value is settable but it does nothing.
    Ok(Value::Undefined)
}

/// Implement `stageHeight`'s getter
pub fn stage_height<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        return Ok(dobj.stage_size().1.into());
    }

    Ok(Value::Undefined)
}

/// Implement `stageHeight`'s setter
pub fn set_stage_height<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    // For some reason this value is settable but it does nothing.
    Ok(Value::Undefined)
}

/// Implement `allowsFullScreen`'s getter
///
/// TODO: This is a stub.
pub fn allows_full_screen<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(true.into())
}

/// Implement `allowsFullScreenInteractive`'s getter
///
/// TODO: This is a stub.
pub fn allows_full_screen_interactive<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(false.into())
}

/// Implement `quality`'s getter
pub fn quality<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let quality = activation.context.stage.quality().into_avm_str();
    Ok(AvmString::from(quality).into())
}

/// Implement `quality`'s setter
pub fn set_quality<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    // Invalid values result in no change.
    if let Ok(quality) = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_string(activation)?
        .parse()
    {
        activation
            .context
            .stage
            .set_quality(activation.context.gc_context, quality);
    }
    Ok(Value::Undefined)
}

/// Construct `Stage`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.display"), "Stage"),
        Some(
            QName::new(
                Namespace::package("flash.display"),
                "DisplayObjectContainer",
            )
            .into(),
        ),
        Method::from_builtin(instance_init, "<Stage instance initializer>", mc),
        Method::from_builtin(class_init, "<Stage class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED);
    write.set_native_instance_init(Method::from_builtin(
        native_instance_init,
        "<Stage native instance initializer>",
        mc,
    ));

    let public_override_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[
        (
            "accessibilityProperties",
            None,
            Some(set_accessibility_properties),
        ),
        ("alpha", None, Some(set_alpha)),
        ("blendMode", None, Some(set_blend_mode)),
        ("cacheAsBitmap", None, Some(set_cache_as_bitmap)),
        ("contextMenu", None, Some(set_context_menu)),
        ("filters", None, Some(set_filters)),
        ("focusRect", None, Some(set_focus_rect)),
        ("loaderInfo", None, Some(set_loader_info)),
        ("mask", None, Some(set_mask)),
        ("mouseEnabled", None, Some(set_mouse_enabled)),
        ("name", Some(name), Some(set_name)),
        ("opaqueBackground", None, Some(set_opaque_background)),
        ("rotation", None, Some(set_rotation)),
        ("scale9Grid", None, Some(set_scale_nine_grid)),
        ("scaleX", None, Some(set_scale_x)),
        ("scaleY", None, Some(set_scale_y)),
        ("scrollRect", None, Some(set_scroll_rect)),
        ("tabEnabled", None, Some(set_tab_enabled)),
        ("tabIndex", None, Some(set_tab_index)),
        ("transform", None, Some(set_transform)),
        ("visible", None, Some(set_visible)),
        ("x", None, Some(set_x)),
        ("y", None, Some(set_y)),
    ];
    for &(name, getter, setter) in public_override_instance_properties {
        if let Some(getter) = getter {
            write.define_instance_trait(
                Trait::from_getter(
                    QName::new(Namespace::public(), name),
                    Method::from_builtin(getter, name, mc),
                )
                .with_override(),
            );
        }
        if let Some(setter) = setter {
            write.define_instance_trait(
                Trait::from_setter(
                    QName::new(Namespace::public(), name),
                    Method::from_builtin(setter, name, mc),
                )
                .with_override(),
            );
        }
    }

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[
        ("align", Some(align), Some(set_align)),
        ("browserZoomFactor", Some(browser_zoom_factor), None),
        ("color", Some(color), Some(set_color)),
        ("contentsScaleFactor", Some(contents_scale_factor), None),
        ("displayState", Some(display_state), Some(set_display_state)),
        ("focus", Some(focus), Some(set_focus)),
        ("frameRate", Some(frame_rate), Some(set_frame_rate)),
        ("scaleMode", Some(scale_mode), Some(set_scale_mode)),
        (
            "showDefaultContextMenu",
            Some(show_default_context_menu),
            Some(set_show_default_context_menu),
        ),
        ("stageWidth", Some(stage_width), Some(set_stage_width)),
        ("stageHeight", Some(stage_height), Some(set_stage_height)),
        ("allowsFullScreen", Some(allows_full_screen), None),
        (
            "allowsFullScreenInteractive",
            Some(allows_full_screen_interactive),
            None,
        ),
        ("quality", Some(quality), Some(set_quality)),
    ];
    write.define_public_builtin_instance_properties(mc, public_instance_properties);

    class
}
