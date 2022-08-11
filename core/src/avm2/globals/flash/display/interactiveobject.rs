//! `flash.display.InteractiveObject` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::{TDisplayObject, TInteractiveObject};
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;

/// Implements `flash.display.InteractiveObject`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("You cannot directly construct InteractiveObject.".into())
}

/// Implements `flash.display.InteractiveObject`'s native instance constructor.
pub fn native_instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;
    }

    Ok(Value::Undefined)
}

/// Implements `flash.display.InteractiveObject`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.mouseEnabled`'s getter.
pub fn mouse_enabled<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        return Ok(int.mouse_enabled().into());
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.mouseEnabled`'s setter.
pub fn set_mouse_enabled<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        let value = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_boolean();
        int.set_mouse_enabled(activation.context.gc_context, value);
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.doubleClickEnabled`'s getter.
pub fn double_click_enabled<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        return Ok(int.double_click_enabled().into());
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.doubleClickEnabled`'s setter.
pub fn set_double_click_enabled<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        let value = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_boolean();
        int.set_double_click_enabled(activation.context.gc_context, value);
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.contextMenu`'s getter.
fn context_menu<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        return Ok(int.context_menu());
    }

    Ok(Value::Undefined)
}

/// Implements `InteractiveObject.contextMenu`'s setter.
fn set_context_menu<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(int) = this
        .and_then(|t| t.as_display_object())
        .and_then(|dobj| dobj.as_interactive())
    {
        let cls_name = QName::new(Namespace::package("flash.display"), "NativeMenu");
        let cls = activation.resolve_class(&cls_name.into())?;
        let value = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_type(activation, cls)?;
        int.set_context_menu(activation.context.gc_context, value);
    }

    Ok(Value::Undefined)
}

/// Construct `InteractiveObject`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.display"), "InteractiveObject"),
        Some(QName::new(Namespace::package("flash.display"), "DisplayObject").into()),
        Method::from_builtin(
            instance_init,
            "<InteractiveObject instance initializer>",
            mc,
        ),
        Method::from_builtin(class_init, "<InteractiveObject class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_native_instance_init(Method::from_builtin(
        native_instance_init,
        "<InteractiveObject native instance initializer>",
        mc,
    ));

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[
        ("mouseEnabled", Some(mouse_enabled), Some(set_mouse_enabled)),
        (
            "doubleClickEnabled",
            Some(double_click_enabled),
            Some(set_double_click_enabled),
        ),
        ("contextMenu", Some(context_menu), Some(set_context_menu)),
    ];
    write.define_public_builtin_instance_properties(mc, public_instance_properties);

    class
}
