//! `flash.display.Shape` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::globals::NS_RUFFLE_INTERNAL;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{Object, StageObject, TObject};
use crate::avm2::traits::Trait;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::Graphic;
use gc_arena::{GcCell, MutationContext};
use ruffle_types::backend::Backend;
use ruffle_types::tag_utils::SwfMovie;
use ruffle_types::vminterface::AvmType;
use std::sync::Arc;

/// Implements `flash.display.Shape`'s instance constructor.
pub fn instance_init<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;

        if this.as_display_object().is_none() {
            let movie = Arc::new(SwfMovie::empty(activation.context.swf.version()));
            let library = activation.context.library.library_for_movie_mut(movie);
            library.force_avm_type(AvmType::Avm2);

            let new_do = Graphic::new_with_avm2(&mut activation.context, this);

            this.init_display_object(activation.context.gc_context, new_do.into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `flash.display.Shape`'s class constructor.
pub fn class_init<'gc, B: Backend>(
    _activation: &mut Activation<'_, 'gc, '_, B>,
    _this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    Ok(Value::Undefined)
}

/// Implements `graphics`.
pub fn graphics<'gc, B: Backend>(
    activation: &mut Activation<'_, 'gc, '_, B>,
    this: Option<Object<'gc, B>>,
    _args: &[Value<'gc, B>],
) -> Result<Value<'gc, B>, Error> {
    if let Some(mut this) = this {
        if let Some(dobj) = this.as_display_object() {
            // Lazily initialize the `Graphics` object in a hidden property.
            let graphics = match this.get_property(
                &QName::new(Namespace::private(NS_RUFFLE_INTERNAL), "graphics").into(),
                activation,
            )? {
                Value::Undefined | Value::Null => {
                    let graphics = Value::from(StageObject::graphics(activation, dobj)?);
                    this.set_property(
                        &QName::new(Namespace::private(NS_RUFFLE_INTERNAL), "graphics").into(),
                        graphics,
                        activation,
                    )?;
                    graphics
                }
                graphics => graphics,
            };
            return Ok(graphics);
        }
    }

    Ok(Value::Undefined)
}

/// Construct `Shape`'s class.
pub fn create_class<'gc, B: Backend>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc, B>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.display"), "Shape"),
        Some(QName::new(Namespace::package("flash.display"), "DisplayObject").into()),
        Method::from_builtin(instance_init, "<Shape instance initializer>", mc),
        Method::from_builtin(class_init, "<Shape class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);

    let public_instance_properties: &[(
        &str,
        Option<NativeMethodImpl<B>>,
        Option<NativeMethodImpl<B>>,
    )] = &[("graphics", Some(graphics), None)];
    write.define_public_builtin_instance_properties(mc, public_instance_properties);

    // Slot for lazy-initialized Graphics object.
    write.define_instance_trait(Trait::from_slot(
        QName::new(Namespace::private(NS_RUFFLE_INTERNAL), "graphics"),
        QName::new(Namespace::package("flash.display"), "Graphics").into(),
        None,
    ));

    class
}
