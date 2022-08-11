//! `flash.text.Font` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{Object, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::character::Character;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;
use ruffle_types::string::AvmString;

/// Implements `flash.text.Font`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;
    }

    Ok(Value::Undefined)
}

/// Implements `flash.text.Font`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `Font.fontName`
pub fn font_name<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some((movie, character_id)) = this.and_then(|this| this.instance_of()).and_then(|this| {
        activation
            .context
            .library
            .avm2_class_registry()
            .class_symbol(this)
    }) {
        if let Some(Character::Font(font)) = activation
            .context
            .library
            .library_for_movie_mut(movie)
            .character_by_id(character_id)
        {
            return Ok(AvmString::new_utf8(
                activation.context.gc_context,
                font.descriptor().class(),
            )
            .into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `Font.fontStyle`
pub fn font_style<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some((movie, character_id)) = this.and_then(|this| this.instance_of()).and_then(|this| {
        activation
            .context
            .library
            .avm2_class_registry()
            .class_symbol(this)
    }) {
        if let Some(Character::Font(font)) = activation
            .context
            .library
            .library_for_movie_mut(movie)
            .character_by_id(character_id)
        {
            return match (font.descriptor().bold(), font.descriptor().italic()) {
                (false, false) => Ok("regular".into()),
                (false, true) => Ok("italic".into()),
                (true, false) => Ok("bold".into()),
                (true, true) => Ok("boldItalic".into()),
            };
        }
    }

    Ok(Value::Undefined)
}

/// Implements `Font.fontType`
pub fn font_type<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some((movie, character_id)) = this.and_then(|this| this.instance_of()).and_then(|this| {
        activation
            .context
            .library
            .avm2_class_registry()
            .class_symbol(this)
    }) {
        if let Some(Character::Font(_)) = activation
            .context
            .library
            .library_for_movie_mut(movie)
            .character_by_id(character_id)
        {
            //TODO: How do we distinguish between CFF and non-CFF embedded fonts?
            return Ok("embedded".into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `Font.hasGlyphs`
pub fn has_glyphs<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some((movie, character_id)) = this.and_then(|this| this.instance_of()).and_then(|this| {
        activation
            .context
            .library
            .avm2_class_registry()
            .class_symbol(this)
    }) {
        let my_str = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;

        if let Some(Character::Font(font)) = activation
            .context
            .library
            .library_for_movie_mut(movie)
            .character_by_id(character_id)
        {
            return Ok(font.has_glyphs_for_str(&my_str).into());
        }
    }

    Ok(Value::Undefined)
}

/// Stub `Font.enumerateFonts`
pub fn enumerate_fonts<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Font.enumerateFonts is a stub".into())
}

/// Stub `Font.registerFont`
pub fn register_font<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Err("Font.registerFont is a stub".into())
}

/// Construct `Font`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.text"), "Font"),
        Some(QName::new(Namespace::package(""), "Object").into()),
        Method::from_builtin(instance_init, "<Font instance initializer>", mc),
        Method::from_builtin(class_init, "<Font class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED);

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[
        ("fontName", Some(font_name), None),
        ("fontStyle", Some(font_style), None),
        ("fontType", Some(font_type), None),
    ];
    write.define_public_builtin_instance_properties(mc, public_instance_properties);

    let public_instance_methods: &[(&str, NativeMethodImpl<B>)] = &[("hasGlyphs", has_glyphs)];
    write.define_public_builtin_instance_methods(mc, public_instance_methods);

    let public_class_methods: &[(&str, NativeMethodImpl<B>)] = &[
        ("enumerateFonts", enumerate_fonts),
        ("registerFont", register_font),
    ];
    write.define_public_builtin_class_methods(mc, public_class_methods);

    class
}
