//! Object representation for TextFormat

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::html::TextFormat;
use gc_arena::{Collect, GcCell, MutationContext};
use ruffle_types::backend::Backend;
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates TextFormat objects.
pub fn textformat_allocator<'gc, B: Backend>(
    class: ClassObject<'gc, B>,
    activation: &mut Activation<'_, 'gc, '_, B>,
) -> Result<Object<'gc, B>, Error> {
    let base = ScriptObjectData::new(class);

    Ok(TextFormatObject(GcCell::allocate(
        activation.context.gc_context,
        TextFormatObjectData {
            base,
            text_format: Default::default(),
        },
    ))
    .into())
}

#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct TextFormatObject<'gc, B: Backend>(GcCell<'gc, TextFormatObjectData<'gc, B>>);

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct TextFormatObjectData<'gc, B: Backend> {
    /// Base script object
    base: ScriptObjectData<'gc, B>,

    text_format: TextFormat,
}

impl<'gc, B: Backend> TextFormatObject<'gc, B> {
    pub fn from_text_format(
        activation: &mut Activation<'_, 'gc, '_, B>,
        text_format: TextFormat,
    ) -> Result<Object<'gc, B>, Error> {
        let class = activation.avm2().classes().textformat;
        let base = ScriptObjectData::new(class);

        let mut this: Object<'gc, B> = Self(GcCell::allocate(
            activation.context.gc_context,
            TextFormatObjectData { base, text_format },
        ))
        .into();
        this.install_instance_slots(activation);

        Ok(this)
    }
}

impl<'gc, B: Backend> TObject<'gc> for TextFormatObject<'gc, B> {
    type B = B;

    fn base(&self) -> Ref<ScriptObjectData<'gc, B>> {
        Ref::map(self.0.read(), |read| &read.base)
    }

    fn base_mut(&self, mc: MutationContext<'gc, '_>) -> RefMut<ScriptObjectData<'gc, B>> {
        RefMut::map(self.0.write(mc), |write| &mut write.base)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc, B>, Error> {
        Ok(Value::Object(Object::from(*self)))
    }

    /// Unwrap this object as a text format.
    fn as_text_format(&self) -> Option<Ref<TextFormat>> {
        Some(Ref::map(self.0.read(), |d| &d.text_format))
    }

    /// Unwrap this object as a mutable text format.
    fn as_text_format_mut(&self, mc: MutationContext<'gc, '_>) -> Option<RefMut<TextFormat>> {
        Some(RefMut::map(self.0.write(mc), |d| &mut d.text_format))
    }
}
