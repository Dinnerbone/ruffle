//! `RegExp` impl

use crate::avm2::class::Class;
use crate::avm2::method::{Method, NativeMethodImpl, ParamConfig};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{regexp_allocator, ArrayObject, Object, TObject};
use crate::avm2::regexp::RegExpFlags;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::avm2::{activation::Activation, array::ArrayStorage};
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;
use ruffle_types::string::{AvmString, WString};

/// Implements `RegExp`'s instance initializer.
pub fn instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;

        if let Some(mut regexp) = this.as_regexp_mut(activation.context.gc_context) {
            regexp.set_source(
                args.get(0)
                    .unwrap_or(&Value::String("".into()))
                    .coerce_to_string(activation)?,
            );

            let flag_chars = args
                .get(1)
                .unwrap_or(&Value::String("".into()))
                .coerce_to_string(activation)?;

            let mut flags = RegExpFlags::empty();
            for c in &flag_chars {
                flags |= match u8::try_from(c) {
                    Ok(b's') => RegExpFlags::DOTALL,
                    Ok(b'x') => RegExpFlags::EXTENDED,
                    Ok(b'g') => RegExpFlags::GLOBAL,
                    Ok(b'i') => RegExpFlags::IGNORE_CASE,
                    Ok(b'm') => RegExpFlags::MULTILINE,
                    _ => continue,
                };
            }

            regexp.set_flags(flags);
        }
    }

    Ok(Value::Undefined)
}

fn class_call<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    let this_class = activation.subclass_object().unwrap();

    if args.len() == 1 {
        let arg = args.get(0).cloned().unwrap();
        if arg.as_object().and_then(|o| o.as_regexp_object()).is_some() {
            return Ok(arg);
        }
    }
    return this_class.construct(activation, args).map(|o| o.into());
}

/// Implements `RegExp`'s class initializer.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `RegExp.dotall`
pub fn dotall<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::DOTALL).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.extended`
pub fn extended<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::EXTENDED).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.global`
pub fn global<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::GLOBAL).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.ignoreCase`
pub fn ignore_case<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::IGNORE_CASE).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.multiline`
pub fn multiline<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::MULTILINE).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.lastIndex`'s getter
pub fn last_index<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(re) = this.as_regexp() {
            return Ok(re.last_index().into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.lastIndex`'s setter
pub fn set_last_index<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(mut re) = this.as_regexp_mut(activation.context.gc_context) {
            let i = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_u32(activation)?;
            re.set_last_index(i as usize);
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.source`
pub fn source<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(re) = this.as_regexp() {
            return Ok(re.source().into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.exec`
pub fn exec<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(mut re) = this.as_regexp_mut(activation.context.gc_context) {
            let text = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_string(activation)?;

            let (storage, index) = match re.exec(text) {
                Some(matched) => {
                    let substrings = matched
                        .groups()
                        .map(|range| range.map(|r| WString::from(&text[r])));

                    let storage = ArrayStorage::from_iter(substrings.map(|s| match s {
                        None => Value::Undefined,
                        Some(s) => AvmString::new(activation.context.gc_context, s).into(),
                    }));

                    (storage, matched.start())
                }
                None => return Ok(Value::Null),
            };

            let object = ArrayObject::from_storage(activation, storage)?;

            object.set_property_local(
                &QName::new(Namespace::public(), "index").into(),
                Value::Number(index as f64),
                activation,
            )?;

            object.set_property_local(
                &QName::new(Namespace::public(), "input").into(),
                text.into(),
                activation,
            )?;

            return Ok(object.into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.test`
pub fn test<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        if let Some(mut re) = this.as_regexp_mut(activation.context.gc_context) {
            let text = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_string(activation)?;
            return Ok(re.test(text).into());
        }
    }

    Ok(Value::Undefined)
}

/// Construct `RegExp`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::public(), "RegExp"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin_and_params(
            instance_init,
            "<RegExp instance initializer>",
            vec![
                ParamConfig::optional("re", QName::new(Namespace::public(), "String").into(), ""),
                ParamConfig::optional(
                    "flags",
                    QName::new(Namespace::public(), "String").into(),
                    "",
                ),
            ],
            false,
            mc,
        ),
        Method::from_builtin(class_init, "<RegExp class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_instance_allocator(regexp_allocator);
    write.set_call_handler(Method::from_builtin(
        class_call,
        "<RegExp call handler>",
        mc,
    ));

    let PUBLIC_INSTANCE_PROPERTIES: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[
        ("dotall", Some(dotall), None),
        ("extended", Some(extended), None),
        ("global", Some(global), None),
        ("ignoreCase", Some(ignore_case), None),
        ("multiline", Some(multiline), None),
        ("lastIndex", Some(last_index), Some(set_last_index)),
        ("source", Some(source), None),
    ];
    write.define_public_builtin_instance_properties(mc, PUBLIC_INSTANCE_PROPERTIES);

    let AS3_INSTANCE_METHODS: &[(&str, NativeMethodImpl<B>)] = &[("exec", exec), ("test", test)];
    write.define_as3_builtin_instance_methods(mc, AS3_INSTANCE_METHODS);

    class
}
