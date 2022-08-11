//! Object impl for boxed values

use crate::avm1::activation::Activation;
use crate::avm1::object::TObject;
use crate::avm1::{Object, ScriptObject, Value};
use crate::impl_custom_object;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::fmt;

/// An Object that serves as a box for a primitive value.
#[derive(Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct ValueObject<'gc, B: Backend>(GcCell<'gc, ValueObjectData<'gc, B>>);

/// The internal data for a boxed value.
#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct ValueObjectData<'gc, B: Backend> {
    /// Base implementation of ScriptObject.
    base: ScriptObject<'gc, B>,

    /// The value being boxed.
    ///
    /// It is a logic error for this to be another object. All extant
    /// constructors for `ValueObject` guard against this by returning the
    /// original object if an attempt is made to box objects.
    value: Value<'gc, B>,
}

impl<'gc, B: Backend> ValueObject<'gc, B> {
    /// Box a value into a `ValueObject`.
    ///
    /// If this function is given an object to box, then this function returns
    /// the already-defined object.
    ///
    /// If a class exists for a given value type, this function automatically
    /// selects the correct prototype for it from the system prototypes list.
    ///
    /// Prefer using `coerce_to_object` instead of calling this function directly.
    pub fn boxed(
        activation: &mut Activation<'_, 'gc, '_, B>,
        value: Value<'gc, B>,
    ) -> Object<'gc, B> {
        if let Value::Object(ob) = value {
            ob
        } else {
            let proto = match &value {
                Value::Bool(_) => Some(activation.context.avm1.prototypes.boolean),
                Value::Number(_) => Some(activation.context.avm1.prototypes.number),
                Value::String(_) => Some(activation.context.avm1.prototypes.string),
                _ => None,
            };

            let obj = ValueObject(GcCell::allocate(
                activation.context.gc_context,
                ValueObjectData {
                    base: ScriptObject::object(activation.context.gc_context, proto),
                    value: Value::Undefined,
                },
            ));

            // Constructor populates the boxed object with the value.
            match &value {
                Value::Bool(_) => {
                    let _ = crate::avm1::globals::boolean::constructor(
                        activation,
                        obj.into(),
                        &[value],
                    );
                }
                Value::Number(_) => {
                    let _ = crate::avm1::globals::number::number(activation, obj.into(), &[value]);
                }
                Value::String(_) => {
                    let _ = crate::avm1::globals::string::string(activation, obj.into(), &[value]);
                }
                _ => (),
            }

            obj.into()
        }
    }

    /// Construct an empty box to be filled by a constructor.
    pub fn empty_box(
        gc_context: MutationContext<'gc, '_>,
        proto: Option<Object<'gc, B>>,
    ) -> Object<'gc, B> {
        ValueObject(GcCell::allocate(
            gc_context,
            ValueObjectData {
                base: ScriptObject::object(gc_context, proto),
                value: Value::Undefined,
            },
        ))
        .into()
    }

    /// Retrieve the boxed value.
    pub fn unbox(self) -> Value<'gc, B> {
        self.0.read().value
    }

    /// Change the value in the box.
    pub fn replace_value(&mut self, gc_context: MutationContext<'gc, '_>, value: Value<'gc, B>) {
        self.0.write(gc_context).value = value;
    }
}

impl<B: Backend> fmt::Debug for ValueObject<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let this = self.0.read();
        f.debug_struct("ValueObject")
            .field("base", &this.base)
            .field("value", &this.value)
            .finish()
    }
}

impl<'gc, B: Backend> TObject<'gc> for ValueObject<'gc, B> {
    type B = B;

    impl_custom_object!(B, base {
        bare_object(as_value_object -> ValueObject::empty_box);
    });
}
